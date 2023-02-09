use std::sync::mpsc;
use std::thread;
use std::env;
use std::process;

mod cli;
mod http;
mod utils;

struct Config {
    url: String,
    session_id: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let url = args[1].clone();
        let session_id = args[2].clone();

        Ok(Config { url, session_id })
    }
}

fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    
    // if args.len() > 0 {
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        
        let (tx, rx) = mpsc::channel();
        for _ in 1..10 {
            let tx = tx.clone();
            let url = config.url.clone();
            let session_id = config.session_id.clone();
            thread::spawn(move || {
                let times = http::custom_request(&url, &session_id);
                tx.send(times).unwrap();
            });
        }
    
        let mut received = rx.recv().unwrap()?;
        println!("MEAN: {}ms", utils::average(&received));
        println!("MEDIAN: {}ms", utils::median(&mut received));
    
        Ok(())
    // } else {
    //     inquire::set_global_render_config(cli::get_render_config());
    //     run_cli();
    // }
}

// fn run(config: Config) -> Result<(), reqwest::Error> {
//     let (tx, rx) = mpsc::channel();
//     for _ in 1..10 {
//         let tx = tx.clone();
//         let base_url = config.base_url.clone();
//         let api_key = config.api_key.clone();
//         let report_name = config.report_name.clone();
//         thread::spawn(move || {
//             let times = http::make_request(&report_name, &api_key, &base_url);
//             tx.send(times).unwrap();
//         });
//     }

//     let mut received = rx.recv().unwrap()?;
//     println!("MEAN: {}ms", utils::average(&received));
//     println!("MEDIAN: {}ms", utils::median(&mut received));
//     println!("MODE: {}ms", utils::mode(&received));

//     Ok(())
// }

// fn run_cli() -> Result<(), reqwest::Error> {
//     let input = cli::get_user_input();
//     println!("\nGetting {} report for this month at {}...", &input["report_name"], &input["base_url"]);
//     let (tx, rx) = mpsc::channel();
//     for _ in 1..10 {
//         let tx = tx.clone();
//         let input = input.clone();
//         thread::spawn(move || {
//             let times = http::make_request(&input["report_name"], &input["api_key"], &input["base_url"]);
//             tx.send(times).unwrap();
//         });
//     }

//     let mut received = rx.recv().unwrap()?;
//     println!("MEAN: {}ms", utils::average(&received));
//     println!("MEDIAN: {}ms", utils::median(&mut received));
//     println!("MODE: {}ms", utils::mode(&received));

//     Ok(())
// }