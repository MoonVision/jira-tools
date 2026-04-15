extern crate env_logger;
extern crate gouqi;

mod printout;
mod stats;

use std::env;
use std::io::Error;

fn create_error(error: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(Error::other(error)))
}

fn print_usage() {
    println!("Usage: jira-tools <command> [args]");
    println!("\nCommands:");
    println!("  stats <JIRA_QUERY>    Show statistics for the given Jira query");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];
    match command.as_str() {
        "stats" => {
            if args.len() < 3 {
                return create_error("Argument JIRA_QUERY required for 'stats' command!");
            }
            let query: String = args[2].clone();
            stats::run_stats(query).await
        }
        _ => {
            println!("Unknown command: {}\n", command);
            print_usage();
            Ok(())
        }
    }
}
