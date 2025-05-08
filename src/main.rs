use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use colored::*;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde_json::{json, Value};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(
    author = "Nitriot (@nitriotsol)",
    version = "1.0.0",
    about = "A powerful tool to test and benchmark Solana RPC endpoints",
    long_about = "Tests various RPC methods and provides detailed performance metrics for Solana RPC endpoints."
)]
struct Args {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "https://mainnet.helius-rpc.com/?api-key=af2cecd4-ff66-48c9-8ef1-fddeb04f3a08")]
    url: String,

    /// Number of iterations for each test
    #[arg(short, long, default_value_t = 3)]
    iterations: usize,

    /// Run tests in parallel
    #[arg(short, long, default_value_t = false)]
    parallel: bool,

    /// Show detailed progress bar
    #[arg(long = "no-progress", action = clap::ArgAction::SetFalse)]
    progress: bool,
}

struct TestResult {
    name: String,
    success: bool,
    duration_ms: u128,
    error: Option<String>,
}

async fn test_get_latest_blockhash(client: &RpcClient) -> Result<TestResult> {
    let start = Instant::now();
    let result = client.get_latest_blockhash().await;
    let duration = start.elapsed();

    match result {
        Ok(_blockhash) => Ok(TestResult {
            name: "getLatestBlockhash".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getLatestBlockhash".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_slot(client: &RpcClient) -> Result<TestResult> {
    let start = Instant::now();
    let result = client.get_slot().await;
    let duration = start.elapsed();

    match result {
        Ok(_slot) => Ok(TestResult {
            name: "getSlot".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getSlot".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_balance(client: &RpcClient) -> Result<TestResult> {
    // Using a known Solana address for testing
    let address = Pubkey::from_str("SoLANAGZJPWXuWQiACz5JJzx1jZKp55FpbjLPwmxA").unwrap_or_default();

    let start = Instant::now();
    let result = client.get_balance(&address).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(TestResult {
            name: "getBalance".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getBalance".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_account_info(client: &RpcClient) -> Result<TestResult> {
    // Using a known Solana address for testing
    let address = Pubkey::from_str("SoLANAGZJPWXuWQiACz5JJzx1jZKp55FpbjLPwmxA").unwrap_or_default();

    let start = Instant::now();
    let result = client.get_account_with_commitment(&address, CommitmentConfig::confirmed()).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(TestResult {
            name: "getAccountInfo".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getAccountInfo".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_block(client: &RpcClient) -> Result<TestResult> {
    // First get the current slot
    let slot_result = client.get_slot().await;

    if let Err(e) = slot_result {
        return Ok(TestResult {
            name: "getBlock".to_string(),
            success: false,
            duration_ms: 0,
            error: Some(format!("Failed to get slot: {}", e)),
        });
    }

    // Use a slot that's a bit older to ensure it's available
    let slot = slot_result.unwrap().saturating_sub(10);

    let start = Instant::now();

    // Use a custom config to handle transaction version
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Base64),
        transaction_details: None,
        rewards: None,
        commitment: None,
        max_supported_transaction_version: Some(0),
    };

    let result = client.get_block_with_config(slot, config).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(TestResult {
            name: "getBlock".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getBlock".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_token_accounts_by_owner(client: &RpcClient) -> Result<TestResult> {
    // Using a known Solana address for testing
    let address = Pubkey::from_str("SoLANAGZJPWXuWQiACz5JJzx1jZKp55FpbjLPwmxA").unwrap_or_default();
    let token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    let start = Instant::now();
    let result = client.get_token_accounts_by_owner(
        &address,
        TokenAccountsFilter::ProgramId(token_program_id)
    ).await;
    let duration = start.elapsed();

    match result {
        Ok(_) => Ok(TestResult {
            name: "getTokenAccountsByOwner".to_string(),
            success: true,
            duration_ms: duration.as_millis(),
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: "getTokenAccountsByOwner".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn test_get_health(url: &str) -> Result<TestResult> {
    let client = Client::new();
    let start = Instant::now();

    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getHealth",
        }))
        .send()
        .await;

    let duration = start.elapsed();

    match response {
        Ok(resp) => {
            let json: Value = resp.json().await?;
            if json["result"] == "ok" {
                Ok(TestResult {
                    name: "getHealth".to_string(),
                    success: true,
                    duration_ms: duration.as_millis(),
                    error: None,
                })
            } else {
                Ok(TestResult {
                    name: "getHealth".to_string(),
                    success: false,
                    duration_ms: duration.as_millis(),
                    error: Some(format!("Unexpected response: {:?}", json)),
                })
            }
        },
        Err(e) => Ok(TestResult {
            name: "getHealth".to_string(),
            success: false,
            duration_ms: duration.as_millis(),
            error: Some(e.to_string()),
        }),
    }
}

async fn run_test(
    test_fn: fn(&RpcClient) -> futures::future::BoxFuture<'_, Result<TestResult>>,
    client: &RpcClient,
    iterations: usize,
    test_name: &str,
    progress_bar: &ProgressBar,
) -> Vec<TestResult> {
    let mut results = Vec::new();

    for i in 0..iterations {
        progress_bar.set_message(format!("Running {} test {}/{}", test_name, i + 1, iterations));

        match test_fn(client).await {
            Ok(result) => results.push(result),
            Err(e) => results.push(TestResult {
                name: test_name.to_string(),
                success: false,
                duration_ms: 0,
                error: Some(e.to_string()),
            }),
        }

        // Add a small delay between tests
        sleep(Duration::from_millis(100)).await;
        progress_bar.inc(1);
    }

    results
}

fn get_speed_rating(duration_ms: u128) -> (&'static str, &'static str) {
    match duration_ms {
        0..=100 => ("Excellent", "bright_green"),
        101..=300 => ("Good", "green"),
        301..=600 => ("Average", "yellow"),
        601..=1000 => ("Slow", "yellow"),
        _ => ("Very Slow", "red"),
    }
}

fn print_test_summary(results: &[TestResult]) {
    if results.is_empty() {
        println!("No test results to display.");
        return;
    }

    // Group results by test name
    let mut grouped_results: std::collections::HashMap<String, Vec<&TestResult>> = std::collections::HashMap::new();
    for result in results {
        grouped_results
            .entry(result.name.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }

    // Calculate overall stats
    let total_tests = results.len();
    let successful_tests = results.iter().filter(|r| r.success).count();
    let overall_success_rate = (successful_tests as f64 / total_tests as f64) * 100.0;

    let overall_avg_duration = if successful_tests > 0 {
        results.iter().filter(|r| r.success).map(|r| r.duration_ms).sum::<u128>() / successful_tests as u128
    } else {
        0
    };

    // Get overall speed rating
    let (speed_rating, rating_color) = get_speed_rating(overall_avg_duration);

    // Print header with fancy box
    println!("\n{}", "╔═══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                      RPC PERFORMANCE REPORT                    ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_blue());

    println!("{}", format!("📊 Timestamp: {}", Utc::now()).dimmed());
    println!("{}", format!("🔍 Overall Success Rate: {:.1}%", overall_success_rate).color(
        if overall_success_rate == 100.0 { "bright_green" }
        else if overall_success_rate >= 80.0 { "green" }
        else if overall_success_rate >= 50.0 { "yellow" }
        else { "red" }
    ));
    println!("{}", format!("⚡ Overall Speed Rating: {} ({} ms avg)",
        speed_rating, overall_avg_duration).color(rating_color));
    println!();

    // Print divider
    println!("{}", "─".repeat(65).dimmed());
    println!();

    // Sort tests by average duration (fastest first)
    let mut sorted_tests: Vec<_> = grouped_results.iter().collect();
    sorted_tests.sort_by(|(_, a_results), (_, b_results)| {
        let a_avg = if !a_results.is_empty() && a_results.iter().any(|r| r.success) {
            a_results.iter().filter(|r| r.success).map(|r| r.duration_ms).sum::<u128>() /
            a_results.iter().filter(|r| r.success).count() as u128
        } else { u128::MAX };

        let b_avg = if !b_results.is_empty() && b_results.iter().any(|r| r.success) {
            b_results.iter().filter(|r| r.success).map(|r| r.duration_ms).sum::<u128>() /
            b_results.iter().filter(|r| r.success).count() as u128
        } else { u128::MAX };

        a_avg.cmp(&b_avg)
    });

    for (test_name, test_results) in sorted_tests {
        let success_count = test_results.iter().filter(|r| r.success).count();
        let total_count = test_results.len();
        let success_rate = (success_count as f64 / total_count as f64) * 100.0;

        let avg_duration: u128 = if success_count > 0 {
            test_results.iter().filter(|r| r.success).map(|r| r.duration_ms).sum::<u128>() / success_count as u128
        } else {
            0
        };

        let min_duration = test_results
            .iter()
            .filter(|r| r.success)
            .map(|r| r.duration_ms)
            .min()
            .unwrap_or(0);

        let max_duration = test_results
            .iter()
            .filter(|r| r.success)
            .map(|r| r.duration_ms)
            .max()
            .unwrap_or(0);

        let status_color = if success_rate == 100.0 {
            "bright_green"
        } else if success_rate >= 80.0 {
            "green"
        } else if success_rate >= 50.0 {
            "yellow"
        } else {
            "red"
        };

        let (speed_rating, rating_color) = get_speed_rating(avg_duration);

        println!(
            "🔹 {} {} ({}/{})",
            test_name.bold(),
            format!("{:.1}%", success_rate).color(status_color),
            success_count,
            total_count
        );

        if success_count > 0 {
            println!(
                "  ⏱️  Response time: avg {}ms | min {}ms | max {}ms",
                avg_duration.to_string().cyan(),
                min_duration.to_string().green(),
                max_duration.to_string().yellow()
            );
            println!(
                "  💨 Speed rating: {}",
                speed_rating.color(rating_color)
            );
        }

        // Print errors if any
        for result in test_results.iter().filter(|r| !r.success) {
            if let Some(error) = &result.error {
                println!("  ❌ {}: {}", "Error".red(), error);
            }
        }

        println!();
    }

    // Print footer
    println!("{}", "═".repeat(65).bright_blue());
    println!("{}", "Thank you for using Solana RPC Performance Checker!".bright_green());
    println!("{}", "Created by Nitriot (@nitriotsol) | Twitter | Telegram: vitualsolana | Discord: nitriot".dimmed());
}

fn print_welcome_screen() {
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                                                               ║".bright_blue());
    println!("{}", "║                SOLANA RPC PERFORMANCE CHECKER                 ║".bright_blue());
    println!("{}", "║                                                               ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_blue());
    println!();
    println!("{}", "A powerful tool to test and benchmark Solana RPC endpoints".yellow());
    println!("{}", "Version 1.0.0 | Created by Nitriot (@nitriotsol)".bright_green());
    println!();
    println!("{}", "This tool will test various RPC methods and provide detailed performance metrics.".cyan());
    println!("{}", "Tests include: getLatestBlockhash, getSlot, getBalance, getAccountInfo,".cyan());
    println!("{}", "getBlock, getTokenAccountsByOwner, and getHealth.".cyan());
    println!();
    println!("{}", "Starting tests in 2 seconds...".green());

    // Sleep for 2 seconds to let the user read the welcome screen
    std::thread::sleep(Duration::from_secs(2));

    println!();
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Show welcome screen
    print_welcome_screen();

    // Print test configuration
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                   TEST CONFIGURATION                          ║".bright_blue());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_blue());
    println!("🔗 RPC endpoint: {}", args.url.cyan());
    println!("🔄 Iterations per test: {}", args.iterations.to_string().yellow());
    println!("⚙️  Mode: {}", if args.parallel { "Parallel".green() } else { "Sequential".yellow() });
    println!();
    println!("{}", "Starting tests now...".green());
    println!();

    let client = RpcClient::new(args.url.clone());

    // Define all the tests
    let tests: Vec<(&str, fn(&RpcClient) -> futures::future::BoxFuture<'_, Result<TestResult>>)> = vec![
        ("getLatestBlockhash", |client| Box::pin(test_get_latest_blockhash(client))),
        ("getSlot", |client| Box::pin(test_get_slot(client))),
        ("getBalance", |client| Box::pin(test_get_balance(client))),
        ("getAccountInfo", |client| Box::pin(test_get_account_info(client))),
        ("getBlock", |client| Box::pin(test_get_block(client))),
        ("getTokenAccountsByOwner", |client| Box::pin(test_get_token_accounts_by_owner(client))),
    ];

    let mut all_results = Vec::new();

    // Only show progress bar if requested
    if args.progress {
        // Create a progress bar
        let total_tests = tests.len() * args.iterations + args.iterations; // +args.iterations for getHealth
        let pb = ProgressBar::new(total_tests as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▓▒░"),
        );

        if args.parallel {
            // Run tests in parallel
            let mut futures = Vec::new();

            for (test_name, test_fn) in tests {
                let client_clone = RpcClient::new(args.url.clone());
                let test_name_clone = test_name.to_string();
                let pb_clone = pb.clone();
                let iterations = args.iterations;

                futures.push(tokio::spawn(async move {
                    run_test(
                        test_fn,
                        &client_clone,
                        iterations,
                        &test_name_clone,
                        &pb_clone,
                    ).await
                }));
            }

            // Also test getHealth separately since it uses a different client
            let url_clone = args.url.clone();
            let pb_clone = pb.clone();
            let iterations = args.iterations;
            futures.push(tokio::spawn(async move {
                let mut results = Vec::new();
                for i in 0..iterations {
                    pb_clone.set_message(format!("Running getHealth test {}/{}", i + 1, iterations));
                    match test_get_health(&url_clone).await {
                        Ok(result) => results.push(result),
                        Err(e) => results.push(TestResult {
                            name: "getHealth".to_string(),
                            success: false,
                            duration_ms: 0,
                            error: Some(e.to_string()),
                        }),
                    }
                    sleep(Duration::from_millis(100)).await;
                    pb_clone.inc(1);
                }
                results
            }));

            // Wait for all tests to complete
            let results = join_all(futures).await;

            // Collect results
            for result in results {
                match result {
                    Ok(test_results) => all_results.extend(test_results),
                    Err(e) => eprintln!("Error running test: {}", e),
                }
            }
        } else {
            // Run tests sequentially
            for (test_name, test_fn) in tests {
                let results = run_test(
                    test_fn,
                    &client,
                    args.iterations,
                    test_name,
                    &pb,
                ).await;

                all_results.extend(results);
            }

            // Also test getHealth
            for i in 0..args.iterations {
                pb.set_message(format!("Running getHealth test {}/{}", i + 1, args.iterations));
                match test_get_health(&args.url).await {
                    Ok(result) => all_results.push(result),
                    Err(e) => all_results.push(TestResult {
                        name: "getHealth".to_string(),
                        success: false,
                        duration_ms: 0,
                        error: Some(e.to_string()),
                    }),
                }
                sleep(Duration::from_millis(100)).await;
                pb.inc(1);
            }
        }

        pb.finish_with_message("Testing completed!");
    } else {
        // Run without progress bar
        println!("Running tests...");

        if args.parallel {
            // Run tests in parallel
            let mut futures = Vec::new();

            for (test_name, test_fn) in tests {
                let client_clone = RpcClient::new(args.url.clone());
                let iterations = args.iterations;

                futures.push(tokio::spawn(async move {
                    let mut results = Vec::new();
                    for _i in 0..iterations {
                        match test_fn(&client_clone).await {
                            Ok(result) => results.push(result),
                            Err(e) => results.push(TestResult {
                                name: test_name.to_string(),
                                success: false,
                                duration_ms: 0,
                                error: Some(e.to_string()),
                            }),
                        }
                        sleep(Duration::from_millis(100)).await;
                    }
                    results
                }));
            }

            // Also test getHealth
            let url_clone = args.url.clone();
            let iterations = args.iterations;
            futures.push(tokio::spawn(async move {
                let mut results = Vec::new();
                for _i in 0..iterations {
                    match test_get_health(&url_clone).await {
                        Ok(result) => results.push(result),
                        Err(e) => results.push(TestResult {
                            name: "getHealth".to_string(),
                            success: false,
                            duration_ms: 0,
                            error: Some(e.to_string()),
                        }),
                    }
                    sleep(Duration::from_millis(100)).await;
                }
                results
            }));

            // Wait for all tests to complete
            let results = join_all(futures).await;

            // Collect results
            for result in results {
                match result {
                    Ok(test_results) => all_results.extend(test_results),
                    Err(e) => eprintln!("Error running test: {}", e),
                }
            }
        } else {
            // Run tests sequentially
            for (test_name, test_fn) in tests {
                for i in 0..args.iterations {
                    print!("Running {} test {}/{}...\r", test_name, i + 1, args.iterations);
                    match test_fn(&client).await {
                        Ok(result) => all_results.push(result),
                        Err(e) => all_results.push(TestResult {
                            name: test_name.to_string(),
                            success: false,
                            duration_ms: 0,
                            error: Some(e.to_string()),
                        }),
                    }
                    sleep(Duration::from_millis(100)).await;
                }
            }

            // Also test getHealth
            for i in 0..args.iterations {
                print!("Running getHealth test {}/{}...\r", i + 1, args.iterations);
                match test_get_health(&args.url).await {
                    Ok(result) => all_results.push(result),
                    Err(e) => all_results.push(TestResult {
                        name: "getHealth".to_string(),
                        success: false,
                        duration_ms: 0,
                        error: Some(e.to_string()),
                    }),
                }
                sleep(Duration::from_millis(100)).await;
            }
        }

        println!("Testing completed!                                ");
    }

    // Print summary
    print_test_summary(&all_results);

    Ok(())
}
