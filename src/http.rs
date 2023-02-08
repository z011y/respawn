use std::collections::HashMap;
use std::time::Instant;
use reqwest;

pub fn make_request(report_name: &String, api_key: &String, base_url: &String) -> Result<Vec<u128>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/api/report-api?apiKey={}", base_url, api_key);
    let start_date = String::from("2022-10-01-00:00");
    let return_type = String::from("json");

    let mut body = HashMap::new();
    body.insert("reportName", report_name);
    body.insert("startDate", &start_date);
    body.insert("returnType", &return_type);

    let mut response_times = Vec::new();

    for _i in 1..10 {
        let now = Instant::now();
        let res = client.post(url.to_string())
            .json(&body)
            .send()?;

        let elapsed_time = now.elapsed();

        // let res = req.send()
        //     .await?
        //     .json::<Vec<HashMap<String, String>>>()
        //     .await?;

        if let reqwest::StatusCode::OK = res.status() {
            println!("{:#?}", elapsed_time)
        } else {
            println!("FAIL");
        }

        response_times.push(elapsed_time.as_millis());
    }

    Ok(response_times)
}