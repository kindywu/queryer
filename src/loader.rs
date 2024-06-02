use crate::DataSet;
use anyhow::{anyhow, Result};
use polars::prelude::*;
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
}

#[derive(Default, Debug)]
pub struct CsvLoader(pub(crate) String);

#[derive(Default, Debug)]
pub struct JsonLoader(pub(crate) String);

impl Loader {
    pub fn load(self) -> Result<DataSet> {
        match self {
            Loader::Csv(csv) => csv.load(),
            Loader::Json(json) => json.load(),
        }
    }
}

pub fn detect_content(data: String, suffix: &str) -> Result<Loader> {
    match suffix {
        // 包括 http / https
        "csv" => Ok(Loader::Csv(CsvLoader(data))),
        // 处理 file://<filename>
        "json" => Ok(Loader::Json(JsonLoader(data))),
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
