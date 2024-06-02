use anyhow::Result;
use queryer::{query, TyrDialect};
use sqlparser::parser::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = "file://juventus.json";

    // 使用 sql 从 URL 里获取数据
    // let sql = format!(
    //     "SELECT Name,Position,Nationality,\"Kit Number\",DOB \
    //     FROM {} where Nationality = 'Italy'",
    //     file
    // );
    let sql = format!(
        "SELECT * \
        FROM {} where Nationality = 'Italy'",
        file
    );

    let ast = Parser::parse_sql(&TyrDialect::default(), &sql);
    println!("{:#?}", ast);

    let df1 = query(sql, "json").await?;
    println!("{:?}", df1);

    Ok(())
}
