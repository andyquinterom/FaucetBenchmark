mod cli;

use clap::Parser;
use itertools::Itertools;
use csv::Writer;
use reqwest::Client;
use tokio::sync::Semaphore;
use std::sync::Arc;

async fn fetch_plumber(http_client: Client, semaphore: Arc<Semaphore>, url: url::Url) -> std::time::Duration {
    // Acquire a permit
    let _permit = semaphore.acquire().await.unwrap();
    // Start of the request
    let start = tokio::time::Instant::now();
    // Make a request
    let _res = http_client.get(url)
        .send()
        .await
        .expect("Failed to send request");
    // Time elapsed start.elapsed()
    start.elapsed()
}

#[derive(serde::Serialize)]
struct BenchmarkResults {
    url: String,
    concurrency: usize,
    weight: usize,
    total_requests: usize,
    total_time: f32,
    average_request_time: f32,
    percentile_25_request_time: f32,
    percentile_50_request_time: f32,
    percentile_75_request_time: f32,
    percentile_95_request_time: f32,
    percentile_99_request_time: f32,
}

async fn run_benchmark(http_client: Client, args: cli::Args, semaphore: Arc<Semaphore>, weight: usize) -> BenchmarkResults {
    let mut tasks = Vec::new();
    // Endpoints
    let endpoints = vec![
        args.get_url("/stress1", weight),
        args.get_url("/stress2", weight),
        args.get_url("/stress3", weight),
    ];
    let start = tokio::time::Instant::now();
    for _ in 0..args.get_iterations() {
        let semaphore = semaphore.clone();
        for endpoint in endpoints.iter() {
            tasks.push(
                tokio::spawn(
                    fetch_plumber(http_client.clone(), semaphore.clone(), endpoint.clone())
                )
            )
        }
    }
    // Wait for all tasks to complete and collect the durations
    let durations = futures::future::join_all(tasks).await;
    // Calculate the total time
    let total_time = start.elapsed();
    // Unwrap the durations
    let durations = durations
        .into_iter()
        .map(|duration| duration.expect("Failed to get duration"))
        .sorted()
        .collect::<Vec<std::time::Duration>>();
    // Calculate average request time
    let average_request_time = durations.iter().sum::<std::time::Duration>() / durations.len() as u32;
    // Calculate the 25th percentile request time
    let percentile_25_request_time = durations[(durations.len() as f32 * 0.25) as usize];
    // Calculate the median request time
    let median_request_time = durations[durations.len() / 2];
    // Calculate the 75th percentile request time
    let percentile_75_request_time = durations[(durations.len() as f32 * 0.75) as usize];
    // Calculate the 95th percentile request time
    let percentile_95_request_time = durations[(durations.len() as f32 * 0.95) as usize];
    // Calculate the 99th percentile request time
    let percentile_99_request_time = durations[(durations.len() as f32 * 0.99) as usize];
    // Turn into benchmark results
    BenchmarkResults {
        url: args.get_url_string(),
        concurrency: args.get_concurrency(),
        weight,
        total_requests: durations.len(),
        total_time: total_time.as_secs_f32(),
        average_request_time: average_request_time.as_secs_f32(),
        percentile_25_request_time: percentile_25_request_time.as_secs_f32(),
        percentile_50_request_time: median_request_time.as_secs_f32(),
        percentile_75_request_time: percentile_75_request_time.as_secs_f32(),
        percentile_95_request_time: percentile_95_request_time.as_secs_f32(),
        percentile_99_request_time: percentile_99_request_time.as_secs_f32(),
    }
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    // The HTTP client
    let http_client = Client::new();
    let semaphore = Arc::new(Semaphore::new(args.get_concurrency()));
    // The async tasks
    let mut results = Vec::new();
    for weight in 1..=10 {
        let weight = weight * 10;
        results.push(
            run_benchmark(http_client.clone(), args.clone(), semaphore.clone(), weight).await
        )
    }
    // Write the results to a CSV to stdout
    let mut writer = Writer::from_writer(std::io::stdout());
    for result in results {
        writer.serialize(result).expect("Failed to serialize result");
    }
}
