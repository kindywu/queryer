use anyhow::{anyhow, Result};
use tokio::{fs, process::Command};

// 从文件源或者 http 源中获取数据，返回字符串
pub async fn retrieve_data(name: String) -> Result<String> {
    match &name[..4] {
        // 包括 http / https
        "http" => UrlFetcher(name).fetch().await,
        // 处理 file://<filename>
        "file" => FileFetcher(name).fetch().await,
        "comm" => StdoutFetcher(name).fetch().await,
        _ => Err(anyhow!("We only support http/https/file at the moment")),
    }
}

pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

struct UrlFetcher(pub(crate) String);
struct FileFetcher(pub(crate) String);
struct StdoutFetcher(pub(crate) String);

impl Fetch for UrlFetcher {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(&self.0).await?.text().await?)
    }
}

impl Fetch for FileFetcher {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}

impl Fetch for StdoutFetcher {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        let cmd = &self.0[7..];
        let output = Command::new(cmd).output().await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout)?;
                    return Ok(stdout);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(anyhow!("tasklist command failed:\n{}", stderr));
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed to execute command: {}", e));
            }
        }
    }
}
