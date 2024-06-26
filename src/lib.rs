use anyhow::{anyhow, Result};
use convert::Sql;
use fetcher::retrieve_data;
use loader::detect_content;
use polars::{
    chunked_array::ops::SortMultipleOptions,
    frame::DataFrame,
    io::{csv::write::CsvWriter, SerWriter},
    lazy::frame::IntoLazy,
};
use sqlparser::parser::Parser;
use tracing::info;

mod convert;
mod dialect;
mod fetcher;
mod loader;

use std::ops::{Deref, DerefMut};

pub use dialect::TyrDialect;

#[derive(Debug)]
pub struct DataSet(DataFrame);

/// 让 DataSet 用起来和 DataFrame 一致
impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 让 DataSet 用起来和 DataFrame 一致
impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    /// 从 DataSet 转换成 csv
    pub fn to_csv(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        let mut writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;
        Ok(String::from_utf8(buf)?)
    }
}

/// 从 from 中获取数据，从 where 中过滤，最后选取需要返回的列
pub async fn query<T: AsRef<str>>(sql: T, suffix: &str) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    // 从 source 读入一个 DataSet
    let data = retrieve_data(source.to_string()).await?;
    let ds = detect_content(data, suffix)?.load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by.into_iter().fold(filtered, |acc, (col, desc)| {
        acc.sort(
            [&col],
            SortMultipleOptions::new().with_order_descending(desc),
        )
    });

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(1000) as u32);
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}
