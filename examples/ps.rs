use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // For windows power shell, we need to run the encoding to utf-8
    // [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
    let file = "comm://tasklist";

    let sql = format!(
        "SELECT * \
        FROM {}",
        file
    );

    println!("{sql}");

    // let ast = sqlparser::parser::Parser::parse_sql(&queryer::TyrDialect::default(), &sql);
    // println!("{:#?}", ast);

    let df = query(sql, "comm").await?;
    println!("{:?}", df);

    Ok(())
}
