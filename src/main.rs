use reqwest::{cookie::Jar, Client};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod days;
use days::run_day;

async fn get_day(day: u16, test: bool) -> Result<String, reqwest::Error> {
    let data_path = format!("data/{}{}.txt", day, if test { "_test" } else { "" });

    if Path::new(data_path.as_str()).is_file() {
        return Ok(fs::read_to_string(data_path).unwrap());
    }
    println!("Downloading data file");

    let session_cookie =
        fs::read_to_string(".session_cookie").expect("Should have been able to read the file");
    let jar = Arc::new(Jar::default());
    let url = "https://adventofcode.com".parse::<reqwest::Url>().unwrap();
    jar.add_cookie_str(format!("session={}", session_cookie).as_str(), &url);

    let client = Client::builder().cookie_provider(jar).build().unwrap();

    let download_url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let response = client.get(download_url).send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        println!("Error: HTTP {}", response.status());
    }

    // Get the response body
    let body = response.text().await?;

    let mut output = File::create(data_path).await.unwrap();
    let _ = output.write(body.as_bytes()).await;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <day> <part>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1].parse::<i16>().unwrap();
    let part = &args[2].parse::<i16>().unwrap();

    let example_input = fs::read_to_string("data/example.txt").unwrap();
    let input = get_day(*day as u16, false).await?;

    let example_output = run_day(*day, *part, example_input);

    println!("Result: Day {} Part {}", day, part);
    println!("Example: {}", example_output);
    if true {
        let output = run_day(*day, *part, input);
        println!("Real: {}", output);
    }
    Ok(())
}
