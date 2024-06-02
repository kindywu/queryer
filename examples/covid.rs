use anyhow::Result;
use polars::io::{csv::read::CsvReader, SerReader};
use polars::prelude::*;
use std::io::Cursor;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file_name = "owid-covid-latest.csv";

    let mut data: String = String::new();
    if let Ok(mut file) = File::open(file_name).await {
        let n = file.read_to_string(&mut data).await?;
        info!("read file size {}", n)
    } else {
        let url = format!(
            "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/{}",
            file_name
        );
        info!("fetch url {}", url);
        data = reqwest::get(url).await?.text().await?;

        let mut file = File::create(file_name).await?;
        file.write_all(data.as_bytes()).await?;
    }

    // 使用 polars 直接请求
    let df = CsvReader::new(Cursor::new(data)).finish()?;
    // println!("{:#?}", df);
    let mask = df.column("new_deaths")?.gt(100)?;
    let filtered = df.filter(&mask)?;
    println!(
        "{:?}",
        filtered.select([
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ])
    );

    Ok(())
}
