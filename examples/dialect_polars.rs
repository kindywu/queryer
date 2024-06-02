use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("owid-covid-latest.csv".into()))?
        .finish()?;

    let filtered  = df.filter(&df["new_deaths"].gt(200)?)?;
    println!("{:?}", filtered.select(["location", "total_cases", "new_cases", "total_deaths", "new_deaths"]));
    Ok(())
}
