use queryer_sql_polars::TyrDialect;
use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    print_sql_ast();

    print_custom_dialect_ast();
}

fn print_custom_dialect_ast() {
    let sql = "SELECT * from https://abc.xyz/covid-cases.csv where new_deaths >= 500";

    let ast = Parser::parse_sql(&TyrDialect::default(), sql);
    println!("{:#?}", ast);
}

fn print_sql_ast() {
    let sql = "SELECT a a1, b, 123, myfunc(b), * 
    FROM data_source 
    WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 
    ORDER BY a DESC, b 
    LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}
