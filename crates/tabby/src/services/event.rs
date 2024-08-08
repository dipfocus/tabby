use std::{path::PathBuf, time::Duration};

use chrono::Utc;
use lazy_static::lazy_static;
use tabby_common::{
    api::event::{EventLogger, LogEntry},
    path,
};
use tokio::{
    io::AsyncWriteExt,
    sync::mpsc::{unbounded_channel, UnboundedSender},
    time::{self},
};
use tracing::error;

lazy_static! {
    /// `WRITER` 是一个静态引用，用于发送字符串。
    ///
    /// 它通过以下方式工作：
    /// 1. 创建 `tx` 和 `rx`，用于发送和接收字符串。
    /// 2. 使用 `tokio::spawn` 启动一个异步任务。
    /// 3. 在异步任务中，创建一个 `EventWriter` 实例，并设置写入文件的路径。
    /// 4. 创建一个时间间隔 `interval`，每隔 5 秒执行一次 `writer.flush().await`。
    /// 5. 进入一个无限循环。
    /// 6. 使用 `tokio::select!` 来选择接收通道中的消息或时间间隔的触发。
    /// 7. 如果接收到消息，调用 `writer.write_line(content).await` 来写入文件。
    /// 8. 如果时间间隔触发，调用 `writer.flush().await` 来刷新缓冲区。
    /// 9. 如果接收到 `None`，表示通道关闭，退出循环。
    static ref WRITER: UnboundedSender<String> = {
        let (tx, mut rx) = unbounded_channel::<String>();

        tokio::spawn(async move {
            let mut writer = EventWriter::new(path::events_dir()).await;
            let mut interval = time::interval(Duration::from_secs(5));

            loop {
                tokio::select! {
                    content = rx.recv() => {
                        if let Some(content) = content {
                            writer.write_line(content).await;
                        } else {
                            break;
                        }
                    }
                    _ = interval.tick() => {
                        writer.flush().await;
                    }
                }
            }
        });

        tx
    };
}

struct EventWriter {
    events_dir: PathBuf,
    filename: Option<String>,
    writer: Option<tokio::io::BufWriter<tokio::fs::File>>,
}

impl EventWriter {
    async fn new(events_dir: PathBuf) -> Self {
        tokio::fs::create_dir_all(events_dir.as_path()).await.ok();

        Self {
            events_dir,
            filename: None,
            writer: None,
        }
    }

    #[cfg(test)]
    fn event_file_path(&self) -> Option<PathBuf> {
        self.filename
            .as_ref()
            .map(|fname| self.events_dir.join(fname))
    }

    /// 异步写入一行内容到事件文件中。
    ///
    /// 参数:
    /// - `content`: 要写入的内容。
    ///
    /// 返回值:
    /// - 无。
    ///
    /// 具体实现:
    /// 1. 获取当前的时间，并使用指定的格式（%Y-%m-%d.json）格式化为字符串。
    /// 2. 如果当前的文件名不等于指定的文件名，则执行以下步骤：
    ///    - 如果存在写入器 `writer`，则刷新缓冲区。
    ///    - 打开或创建一个新的事件文件，并创建一个写入器 `writer`。
    ///    - 更新文件名和写入器。
    /// 3. 获取写入器 `writer`，并写入指定的内容，并在末尾添加换行符。
    ///
    /// 注意:
    /// - 此方法使用异步操作，需要在异步环境中调用。
    async fn write_line(&mut self, content: String) {
        let now = Utc::now();
        let fname = now.format("%Y-%m-%d.json");

        if self.filename != Some(fname.to_string()) {
            if let Some(mut w) = self.writer.take() {
                w.flush().await.unwrap();
            }

            let file = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open(self.events_dir.join(fname.to_string()))
                .await
                .ok()
                .unwrap();
            self.writer = Some(tokio::io::BufWriter::new(file));
            self.filename = Some(fname.to_string());
        }

        let writer = self.writer.as_mut().unwrap();
        writer
            .write_all(format!("{}\n", content).as_bytes())
            .await
            .unwrap();
    }

    async fn flush(&mut self) {
        let writer = self.writer.as_mut().unwrap();
        writer.flush().await.unwrap()
    }
}

struct EventService;

impl EventLogger for EventService {
    /// 将日志条目序列化为 JSON 格式，并将其写入文件。
    ///
    /// 参数:
    /// - `x`: 要写入文件的日志条目。
    ///
    /// 返回值:
    /// 无。
    fn write(&self, x: LogEntry) {
        let json = match serdeconv::to_json_string(&x) {
            Ok(json) => json,
            Err(err) => {
                error!("Failed to serialize event into json {}", err);
                return;
            }
        };

        if let Err(err) = WRITER.send(json) {
            error!("Failed to write event to file: {}", err);
        }
    }
}

#[allow(unused)]
pub fn create_event_logger() -> impl EventLogger + 'static {
    EventService
}

#[cfg(test)]
mod tests {
    use super::*;

    fn events_dir() -> PathBuf {
        std::env::temp_dir().join(".tabby").join("events")
    }

    async fn test_event_writer_swap_file() {
        tokio::fs::create_dir_all(events_dir()).await.ok();

        let old_fname = "2021-01-01.json".to_string();
        let old_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(events_dir().join(old_fname.clone()))
            .await
            .ok()
            .unwrap();
        let mut old_wr = tokio::io::BufWriter::new(old_file);
        old_wr
            .write_all(format!("{}\n", "old data in old file").as_bytes())
            .await
            .unwrap();
        old_wr.flush().await.unwrap();

        let mut event_wr = EventWriter {
            events_dir: events_dir(),
            filename: Some(old_fname.clone()),
            writer: Some(old_wr),
        };
        event_wr.write_line("test data".to_string()).await;
        event_wr.flush().await;

        // we should be able to read new created file successfully
        let content = tokio::fs::read_to_string(event_wr.event_file_path().unwrap())
            .await
            .unwrap();
        assert_eq!(content.as_str(), "test data\n");
        // old file should have no more writes
        let content = tokio::fs::read_to_string(events_dir().join(old_fname))
            .await
            .unwrap();
        assert_eq!(content.as_str(), "old data in old file\n");
    }

    #[tokio::test]
    async fn test_event_writer() {
        // in case previous test failed
        tokio::fs::remove_dir_all(events_dir()).await.ok();

        test_event_writer_swap_file().await;
        tokio::fs::remove_dir_all(events_dir()).await.unwrap();
    }
}
