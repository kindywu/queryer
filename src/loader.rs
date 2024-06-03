use crate::DataSet;
use anyhow::{anyhow, Result};
use polars::prelude::*;
use regex::Regex;
use std::io::Cursor;

pub trait Load {
    type Error;
    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
// #[non_exhaustive]
pub enum Loader {
    Csv(CsvLoader),
    Json(JsonLoader),
    Tsv(TsvLoader),
}

#[derive(Default, Debug)]
pub struct CsvLoader(pub(crate) String);

#[derive(Default, Debug)]
pub struct JsonLoader(pub(crate) String);

#[derive(Default, Debug)]
pub struct TsvLoader(pub(crate) String);

impl Loader {
    pub fn load(self) -> Result<DataSet> {
        match self {
            Loader::Csv(csv) => csv.load(),
            Loader::Json(json) => json.load(),
            Loader::Tsv(tsv) => tsv.load(),
        }
    }
}

pub fn detect_content(data: String, suffix: &str) -> Result<Loader> {
    match suffix {
        // 包括 http / https
        "csv" => Ok(Loader::Csv(CsvLoader(data))),
        // 处理 file://<filename>
        "json" => Ok(Loader::Json(JsonLoader(data))),
        "comm" => Ok(Loader::Tsv(TsvLoader(data))),
        _ => Err(anyhow!("We only support csv/json at the moment")),
    }
}

impl Load for CsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0)).finish()?;
        Ok(DataSet(df))
    }
}

impl Load for JsonLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = JsonReader::new(Cursor::new(self.0)).finish()?;
        Ok(DataSet(df))
    }
}

impl Load for TsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = parse_command_output(&self.0)?;
        Ok(DataSet(df))
    }
}
fn parse_command_output(output: &str) -> Result<DataFrame> {
    let lines = output.lines();

    // 解析每一行数据
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut headers: Vec<&str> = Vec::new();
    for line in lines {
        // 跳过空行
        if line.trim().is_empty() {
            continue;
        }

        // 跳过分割行
        if line.starts_with("====") {
            continue;
        }

        // 获取标题行
        if headers.is_empty() {
            let re = Regex::new(r"(\s{2,})").unwrap(); // 匹配两个或以上的空格
            headers = re.split(line).collect();
        }

        // 使用正则表达式来正确解析列
        let re = Regex::new(r"\s+").unwrap();
        let row: Vec<String> = re.split(line).map(String::from).collect();
        data.push(row);
    }

    // 将数据转换为 polars DataFrame
    let mut columns = Vec::with_capacity(headers.len());
    for i in 0..headers.len() {
        let col_data: Vec<&str> = data
            .iter()
            .map(|row| row.get(i).map(String::as_str).unwrap_or(""))
            .collect();
        columns.push(Series::new(headers[i], col_data));
    }

    let df = DataFrame::new(columns)?;
    Ok(df)
}
