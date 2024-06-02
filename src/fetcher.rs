use anyhow::{anyhow, Result};
use tokio::fs;

// 从文件源或者 http 源中获取数据，返回字符串
pub async fn retrieve_data(name: String) -> Result<String> {
    match &name[..4] {
        // 包括 http / https
        "http" => UrlFetcher(name).fetch().await,
        // 处理 file://<filename>
        "file" => FileFetcher(name).fetch().await,
        _ => Err(anyhow!("We only support http/https/file at the moment")),
    }
}

pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

struct UrlFetcher(pub(crate) String);
struct FileFetcher(pub(crate) String);

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
