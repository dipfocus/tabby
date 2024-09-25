use std::sync::Arc;
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use futures::{stream::BoxStream, StreamExt};
use tracing::{debug, warn, error};
use futures::Stream;
use async_stream::stream;
use anyhow::{anyhow, Result};
use octocrab::Octocrab;

use tabby_common::config::CodeRepository;
use tabby_index::public::{CodeIndexer, WebDocument};
use tabby_inference::Embedding;
use tabby_schema::{job::JobService, repository::GitRepositoryService};

use super::{helper::Job, BackgroundJobEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchedulerGitJob {
    repository: CodeRepository,
}

impl SchedulerGitJob {
    pub fn new(repository: CodeRepository) -> Self {
        Self { repository }
    }
}

impl Job for SchedulerGitJob {
    const NAME: &'static str = "scheduler_git";
}

impl SchedulerGitJob {
    pub async fn run(
        self, 
        embedding: Arc<dyn Embedding>,
        git_repository_service: Arc<dyn GitRepositoryService>
    ) -> tabby_schema::Result<()> {
        let repository = self.repository.clone();
        
        let s: BoxStream<(DateTime<Utc>, WebDocument)> = fetch_github_issues(
                &self.repository.source_id,
                &self.repository.git_url,
            )
            .await?
            .boxed();

        stream! {
            let mut count = 0;
            let mut num_updated = 0;
            for await (updated_at, doc) in s {
                match git_repository_service.add_issue(
                    self.repository.source_id.as_str(), 
                    doc.link.as_str(), 
                    doc.title.as_str(), 
                    doc.body.as_str()
                ).await {
                    Ok(true) => num_updated += 1,
                    Ok(false) => (),
                    Err(e) => {
                        error!("Failed to add issue: {}", e);
                    }
                }
                count += 1;
                if count % 100 == 0 {
                    logkit::info!("{} docs seen, {} docs updated", count, num_updated);
                    debug!("{} docs seen, {} docs updated", count, num_updated);
                };
            }

            logkit::info!("{} docs seen, {} docs updated", count, num_updated);
            debug!("{} docs seen, {} docs updated", count, num_updated);
        }
        .count()
        .await;
        tokio::spawn(async move {
            let mut code = CodeIndexer::default();
            code.refresh(embedding, &repository).await
        })
        .await
        .context("Job execution failed")??;
        Ok(())
    }

    pub async fn cron(
        _now: DateTime<Utc>,
        git_repository: Arc<dyn GitRepositoryService>,
        job: Arc<dyn JobService>,
    ) -> tabby_schema::Result<()> {
        let repositories = git_repository
            .repository_list()
            .await
            .context("Must be able to retrieve repositories for sync")?;

        debug!("Starting to schedule git jobs, found {} repositories", repositories.len());
        let repositories: Vec<_> = repositories
            .into_iter()
            .map(|repo| CodeRepository::new(&repo.git_url, &repo.source_id))
            .collect();

        for repository in repositories {
            let _ = job
                .trigger(BackgroundJobEvent::SchedulerGitRepository(repository).to_command())
                .await;
        }
        Ok(())
    }
}

pub async fn fetch_github_issues(
    source_id: &str,
    git_url: &str,
) -> Result<impl Stream<Item = (DateTime<Utc>, WebDocument)>> {
    let octocrab = Octocrab::builder() .build()?;
    // Example repository URL: https://github.com/owner/repo

    let parts: Vec<&str> = git_url.rsplitn(3, '/').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid GitHub repository URL format"));
    }
    let [repo, owner, _] = parts[..] else {
        return Err(anyhow::anyhow!("Failed to destructure URL parts"));
    };
    debug!("Fetching github issues for {}, owner: {}, repo: {}", git_url, owner, repo);

    let (owner, repo) = git_url
        .split_once('/')
        .ok_or_else(|| anyhow!("Invalid repository URL"))?;

    let owner = owner.to_owned();
    let repo = repo.to_owned();
    let source_id = source_id.to_owned();
    let mut total_count = 0;
    let s = stream! {
        let mut page = 1u32;
        loop {
            let response = match octocrab
                .issues(&owner, &repo)
                .list()
                .state(octocrab::params::State::Open)
                .page(page)
                .send()
                .await {
                    Ok(x) => x,
                    Err(e) => {
                        logkit::error!("Failed to fetch issues: {}", e);
                        break;
                    }
            };
            total_count += response.items.len();

            let pages = response.number_of_pages().unwrap_or_default();

            for issue in response.items {
                let doc = WebDocument {
                    source_id: source_id.to_string(),
                    id: issue.html_url.to_string(),
                    link: issue.html_url.to_string(),
                    title: issue.title,
                    body: issue.body.unwrap_or_default(),
                };
                yield (issue.updated_at, doc);
            }

            page += 1;
            if page > pages {
                break;
            }
        }
    };
    debug!("{} issues fetched", total_count);
    Ok(s)
}
