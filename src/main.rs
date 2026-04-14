extern crate env_logger;
extern crate gouqi;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Table};
use futures::stream::StreamExt;
use gouqi::{Credentials, SearchOptions};
use owo_colors::OwoColorize;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::Error;

fn create_error(error: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(Error::other(error)))
}

#[derive(Deserialize, Debug)]
struct JiraField {
    // custom: bool,
    id: String,
    // key: String,
    name: String,
}

fn larger_option_number<N: std::cmp::PartialOrd>(a: Option<N>, b: Option<N>) -> Option<N> {
    match a {
        Some(a_v) => match b {
            Some(b_v) => {
                if a_v > b_v {
                    Some(a_v)
                } else {
                    Some(b_v)
                }
            }
            None => Some(a_v),
        },
        None => b,
    }
}

fn print_stats_for_humans(
    issue_count_by_status: &HashMap<String, u32>,
    issue_count_by_issue_type: &HashMap<String, u32>,
    story_points_by_status: &HashMap<String, f64>,
    story_points_by_issue_type: &HashMap<String, f64>,
    story_points_unset_by_issue_type: &HashMap<String, u32>,
    total_story_points: f64,
    total_issue_count: u32,
) {
    println!("\n{}", "Issue count by issue type:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Issue Type", "Count"]);
    for (issue_type, issue_count) in issue_count_by_issue_type.iter() {
        table.add_row(vec![
            Cell::new(issue_type),
            Cell::new(issue_count).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Story points by issue type:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Issue Type", "Story Points"]);
    for (issue_type, story_points) in story_points_by_issue_type.iter() {
        table.add_row(vec![
            Cell::new(issue_type),
            Cell::new(story_points).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Unset SPs by issue type:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Issue Type", "Unset Count"]);
    for (issue_type, unset_count) in story_points_unset_by_issue_type.iter() {
        table.add_row(vec![
            Cell::new(issue_type),
            Cell::new(unset_count).set_alignment(CellAlignment::Right),
        ]);
    }
    if story_points_unset_by_issue_type.is_empty() {
        println!("No issues with unset story points.");
    } else {
        println!("{table}");
    }

    println!("\n{}", "Issue count by status:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Status", "Count"]);
    for (status, issue_count) in issue_count_by_status.iter() {
        table.add_row(vec![
            Cell::new(status),
            Cell::new(issue_count).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Story points by status:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Status", "Story Points"]);
    for (status, story_points) in story_points_by_status.iter() {
        table.add_row(vec![
            Cell::new(status),
            Cell::new(story_points).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Totals:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Metric", "Total"]);
    table.add_row(vec![
        Cell::new("Issue Count"),
        Cell::new(total_issue_count).set_alignment(CellAlignment::Right),
    ]);
    table.add_row(vec![
        Cell::new("Story Points"),
        Cell::new(total_story_points).set_alignment(CellAlignment::Right),
    ]);
    println!("{table}");
}

#[derive(Debug, Deserialize)]
struct JiraToolsConfig {
    host: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_target(false)
    //     .init();
    let xdg_dirs = xdg::BaseDirectories::with_prefix("jira-tools");
    let xdg_config_path = xdg_dirs.get_config_file("jira-tools.toml").unwrap();
    let xdg_config = xdg_config_path.to_str().unwrap_or("jira-tools.toml");
    let cb: Result<JiraToolsConfig, config::ConfigError> = config::Config::builder()
        .add_source(config::File::with_name("jira-tools.toml").required(false))
        .add_source(config::File::with_name(xdg_config).required(false))
        .add_source(config::Environment::with_prefix("JIRA"))
        .build()?
        .try_deserialize();
    let conf = cb.expect("Env variables JIRA_HOST, JIRA_USERNAME & JIRA_PASSWORD need to be set!");

    let jira_host = conf.host;
    let jira_user = conf.username;
    let jira_pass = conf.password;
    let field_endpoint = format!("{jira_host}/rest/api/3/field");
    let resp = reqwest::Client::new()
        .get(field_endpoint)
        .basic_auth(jira_user.clone(), Some(jira_pass.clone()))
        .send()
        .await?
        .json::<Vec<JiraField>>()
        .await?;

    let mut story_point_fields: HashMap<String, &JiraField> = HashMap::new();
    resp.iter()
        .filter(|jira_field| jira_field.name.to_lowercase().contains("story point"))
        .for_each(|jira_field| {
            story_point_fields.insert(jira_field.id.clone(), jira_field);
        });

    let first_arg = env::args().nth(1);
    if first_arg.is_none() {
        return create_error("Argument JIRA_QUERY required!");
    }
    let query: String = env::args().nth(1).unwrap();
    let jira = gouqi::r#async::Jira::new(jira_host, Credentials::Basic(jira_user, jira_pass))
        .expect("Error initializing Jira");

    let mut issue_count_by_status: HashMap<String, u32> = HashMap::new();
    let mut issue_count_by_issue_type: HashMap<String, u32> = HashMap::new();
    let mut story_points_by_status: HashMap<String, f64> = HashMap::new();
    let mut story_points_by_issue_type: HashMap<String, f64> = HashMap::new();
    let mut story_points_unset_by_issue_type: HashMap<String, u32> = HashMap::new();
    let mut total_story_points: f64 = 0.0;
    let mut total_issue_count: u32 = 0;

    let search_options = SearchOptions::builder().all_fields().build();

    let binding = jira.search();
    let mut stream = binding
        .stream(query, &search_options)
        .await
        .expect("Error starting search stream");
    while let Some(issue) = stream.next().await {
        print!("{} - ", issue.key.blue().bold());
        let mut story_points: Option<f64> = None;
        for (key, value) in issue.fields.iter() {
            if story_point_fields.contains_key(key) {
                story_points = larger_option_number(story_points, value.as_f64());
            }
        }
        if let Some(issue_type) = issue.issue_type() {
            issue_count_by_issue_type.insert(
                issue_type.name.clone(),
                issue_count_by_issue_type
                    .get(&issue_type.name)
                    .unwrap_or(&0)
                    + 1,
            );
            print!("{} - ", issue_type.name.yellow());
            if issue_type.name.to_lowercase().contains("epic") {
                println!();
                continue;
            }
            story_points_by_issue_type.insert(
                issue_type.name.clone(),
                story_points_by_issue_type
                    .get(&issue_type.name)
                    .unwrap_or(&0.0)
                    + story_points.unwrap_or(0.0),
            );
            if story_points.is_none() {
                story_points_unset_by_issue_type.insert(
                    issue_type.name.clone(),
                    story_points_unset_by_issue_type
                        .get(&issue_type.name)
                        .unwrap_or(&0)
                        + 1,
                );
            }
        }
        if let Some(status) = issue.status() {
            let status_name = if status.name.to_lowercase().contains("review") {
                String::from("Review")
            } else {
                status.name.clone()
            };
            println!("{} ", status.name.green());
            issue_count_by_status.insert(
                status_name.clone(),
                issue_count_by_status.get(&status_name).unwrap_or(&0) + 1,
            );
            story_points_by_status.insert(
                status_name.clone(),
                story_points_by_status.get(&status_name).unwrap_or(&0.0)
                    + story_points.unwrap_or(0.0),
            );
        }
        total_issue_count += 1;
        total_story_points += story_points.unwrap_or(0.0);
    }

    print_stats_for_humans(
        &issue_count_by_status,
        &issue_count_by_issue_type,
        &story_points_by_status,
        &story_points_by_issue_type,
        &story_points_unset_by_issue_type,
        total_story_points,
        total_issue_count,
    );
    Ok(())
}
