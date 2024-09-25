use anyhow::{Result};
use sqlx::{prelude::FromRow, query};
use tabby_db_macros::query_paged_as;

use crate::{DbConn, SQLXResultExt};

#[derive(FromRow)]
pub struct IssueDAO {
    pub id: i64,
    pub source_id: String,
    pub url: String,
    pub title: String,
    pub description: String,
}

impl DbConn {
    pub async fn add_issue(
        &self, 
        source_id: &str, 
        url: &str, 
        title: &str, 
        description: &str
    ) -> Result<bool> {
        let res = query!(
            "INSERT INTO issues (source_id, url, title, description) VALUES (?, ?, ?, ?)",
            source_id,
            url,
            title,
            description
        )
        .execute(&self.pool)
        .await;
        
        res.unique_error("Failed to add issue, url already exists").map(|_| true)
    }

    pub async fn list_issues(
        &self, 
        source_id: &str,
        limit: Option<usize>,
        skip_id: Option<i32>,
        backwards: bool,
    ) -> Result<Vec<IssueDAO>> {
        let condition = Some(format!("source_id = '{source_id}'"));
        let issues = query_paged_as!(
            IssueDAO,
            "issues",
            ["id", "source_id", "url", "title", "description"],
            limit,
            skip_id,
            backwards,
            condition
        ).fetch_all(&self.pool).await?;
        Ok(issues)
    }

    pub async fn get_issue(&self, url: &str) -> Result<IssueDAO> {
        let issue = sqlx::query_as!(
            IssueDAO, 
            "SELECT id as 'id!', source_id, url, title, description FROM issues WHERE url = ?", 
            url).fetch_one(&self.pool).await?;
        Ok(issue)
    }

}

#[cfg(test)]
mod tests {
    use crate::DbConn;

    #[tokio::test]
    async fn test_add_issue() {
        let conn = DbConn::new_in_memory().await.unwrap();
        let issue1 = conn
            .add_issue("source_id1", "url1", "title1", "description1")
            .await
            .unwrap();
        assert!(issue1);

        let issue2 = conn
            .add_issue("source_id2", "url2", "title2", "description2")
            .await
            .unwrap();
        assert!(issue2);

        let issues = conn
            .list_issues("source_id1", None, None, false)
            .await
            .unwrap();
        assert!(issues.len() == 1);

        let issue = conn.get_issue("url1").await.unwrap();
        assert_eq!(issue.url, "url1");
    }
}