use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

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
