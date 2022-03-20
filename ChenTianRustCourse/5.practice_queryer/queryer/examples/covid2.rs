use anyhow::Result;
use queryer::query;
use queryer::PROXY_URL_PREFIX;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = format!("{}{}", PROXY_URL_PREFIX, "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv");

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC",
        url
    );

    let df1 = query(sql).await?;

    println!("{:?}", df1);

    Ok(())
}