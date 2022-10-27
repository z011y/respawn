use std::{ io, collections::HashMap };
use reqwest;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // command prompts
    println!("Base URL:");
    let mut base_url = String::new();
    io::stdin()
        .read_line(&mut base_url)
        .expect("Invalid URL");
    base_url.pop(); // remove trailing newline

    println!("Name of the report to run:");
    let mut report_name = String::new();
    io::stdin()
        .read_line(&mut report_name)
        .expect("Invalid report name");
    report_name.pop(); // remove trailing newline

    println!("Getting {} report for this month at {}...", report_name.bold(), base_url.bold());

    // request
    let client = reqwest::Client::new();
    let url = format!("{}/api/report-api?apiKey=1697b2ac76144ee09905829c918c6bc5", base_url).to_string();

    let mut body = HashMap::new();
    body.insert("reportName", report_name);
    body.insert("startDate", "2022-10-01-00:00".to_string());
    body.insert("returnType", "json".to_string());

    let req = client
        .post(url)
        .json(&body);

    let res = req.send()
        .await?
        .json::<Vec<HashMap<String, String>>>()
        .await?;

    println!("{:#?}", res);

    Ok(())
}