use std::collections::HashMap;
use std::time::{Instant, Duration};
use reqwest;
use reqwest::header::{COOKIE};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
    report_view: ReportView
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportView {
    filter: Filter,
    report_id: u32,
    time_zone: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Filter {
    is_utc_time: bool,
    user_time_range_option: u32,
    table_args: TableArgs
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TableArgs {
    columns_to_include: Vec<String>
}

pub fn report_api_request(report_name: &String, api_key: &String, base_url: &String) -> Result<Vec<u128>, reqwest::Error> {
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

pub fn custom_request(url: &String, session_id: &String) -> Result<Vec<u128>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let report_view = r#"{
        "reportView": {
          "filter": {
            "filterOptions": [],
            "isUtcTime": true,
            "tableArgs": {
              "columnsToInclude": [
                "DATE",
                "FROM FULL NAME",
                "TO FULL NAME",
                "POINTS AWARDED",
                "PROGRAM NAME",
                "TAGS"
              ]
            },
            "userTimeRangeOption": 11
          },
          "reportId": 5,
          "timeZone": "UTC"
        }
      }"#;
    let body: RequestBody = serde_json::from_str(&report_view).unwrap();
    let cookie_value = ".ASPXAUTH=".to_owned() + session_id;
    let mut response_times = Vec::new();

    for _i in 1..10 {
        let cookie_value = cookie_value.clone();
        let anti_forgery_token = "9p8N4aNdFXbj75aBMrQO5wqvDVY1GjBD4ppjUZPITCbwhcJj4rLOPKdjHCRVWnSPG00gkbpRSHZPkGVl591E9Zk1xj54ZpmAG7ar3M-zeqY1:I5fkHKXQlU7u8xuUn3AToXu_E3kigz8Buqh56-PowHlzG7m9HxVsWGB6J3L7XHc80YAnUS44ex8wyRQPAiE4gZxpWIaMubSyAcj4WW17F0w1";
        let now = Instant::now();
        let res = client
            .post(url.to_string())
            .timeout(Duration::from_secs(240))
            .json(&body)
            .header(COOKIE, cookie_value)
            .header("X-Anti-Forgery-Token", anti_forgery_token)
            .send()?;

        let elapsed_time = now.elapsed();

        // let res = req.send()
        //     .await?
        //     .json::<Vec<HashMap<String, String>>>()
        //     .await?;

        if let reqwest::StatusCode::OK = res.status() {
            println!("{:#?}", elapsed_time)
        } else {
            println!("FAIL, status code: {}, url: {}, message: {:#?}", res.status(), url, res.text());
        }

        response_times.push(elapsed_time.as_millis());
    }

    Ok(response_times)
}