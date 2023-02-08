use std::sync::mpsc;
use std::thread;
use std::env;
use std::process;

mod cli;
mod http;
mod utils;

struct Config {
    base_url: String,
    api_key: String,
    report_name: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let base_url = args[1].clone();
        let api_key = args[2].clone();
        let report_name = args[3].clone();

        Ok(Config { base_url, api_key, report_name })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 0 {
        let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        run(config);
    } else {
        inquire::set_global_render_config(cli::get_render_config());
        run_cli();
    }
}

fn run(config: Config) -> Result<(), reqwest::Error> {
    let (tx, rx) = mpsc::channel();
    for _ in 1..10 {
        let tx = tx.clone();
        let base_url = config.base_url.clone();
        let api_key = config.api_key.clone();
        let report_name = config.report_name.clone();
        thread::spawn(move || {
            let times = http::make_request(&report_name, &api_key, &base_url);
            tx.send(times).unwrap();
        });
    }

    let mut received = rx.recv().unwrap()?;
    println!("MEAN: {}ms", utils::average(&received));
    println!("MEDIAN: {}ms", utils::median(&mut received));
    println!("MODE: {}ms", utils::mode(&received));

    Ok(())
}

fn run_cli() -> Result<(), reqwest::Error> {
    let input = cli::get_user_input();
    println!("\nGetting {} report for this month at {}...", &input["report_name"], &input["base_url"]);
    let (tx, rx) = mpsc::channel();
    for _ in 1..10 {
        let tx = tx.clone();
        let input = input.clone();
        thread::spawn(move || {
            let times = http::make_request(&input["report_name"], &input["api_key"], &input["base_url"]);
            tx.send(times).unwrap();
        });
    }

    let mut received = rx.recv().unwrap()?;
    println!("MEAN: {}ms", utils::average(&received));
    println!("MEDIAN: {}ms", utils::median(&mut received));
    println!("MODE: {}ms", utils::mode(&received));

    Ok(())
}