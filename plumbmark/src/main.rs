mod cli;

use clap::Parser;
use itertools::Itertools;
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
    // Time elapsed
    start.elapsed()
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    // The HTTP client
    let http_client = Client::new();
    let semaphore = Arc::new(Semaphore::new(args.get_concurrency()));
    // The async tasks
    let mut tasks = Vec::new();
    // Endpoints
    let endpoints = vec![
        args.get_url("/stress1"),
        args.get_url("/stress2"),
        args.get_url("/stress3"),
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
    // Calculate the median request time
    let median_request_time = durations[durations.len() / 2];
    // Calculate the 95th percentile request time
    let percentile_95_request_time = durations[(durations.len() as f32 * 0.95) as usize];
    // Calculate the 99th percentile request time
    let percentile_99_request_time = durations[(durations.len() as f32 * 0.99) as usize];
    // Print the results
    println!("Total requests: {}", durations.len());
    println!("Total time: {:?}", total_time);
    println!("Average request time: {:?}", average_request_time);
    println!("Median request time: {:?}", median_request_time);
    println!("95th percentile request time: {:?}", percentile_95_request_time);
    println!("99th percentile request time: {:?}", percentile_99_request_time);
}
