use std::collections::HashMap;

use futures::stream::StreamExt;
use gouqi::{Credentials, SearchOptions};
use serde_derive::Deserialize;

use crate::printout;

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

#[derive(Debug, Deserialize)]
struct JiraToolsConfig {
    host: String,
    username: String,
    password: String,
}

pub async fn run_stats(query: String) -> Result<(), Box<dyn std::error::Error>> {
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
        .await?;

    if let Some(reason) = resp.headers().get("x-seraph-loginreason") {
        if reason == "AUTHENTICATED_FAILED" {
            return Err("Jira authentication failed (ensure you use your Atlassian account email address and a valid API token).
You can provide credentials via:
1. Environment variables: JIRA_USERNAME and JIRA_PASSWORD
2. XDG config file: ~/.config/jira-tools/jira-tools.toml".into());
        }
    }

    let resp_fields = resp.json::<Vec<JiraField>>().await?;

    let mut story_point_fields: HashMap<String, &JiraField> = HashMap::new();
    resp_fields.iter()
        .filter(|jira_field| jira_field.name.to_lowercase().contains("story point"))
        .for_each(|jira_field| {
            story_point_fields.insert(jira_field.id.clone(), jira_field);
        });

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
        let mut story_points: Option<f64> = None;
        for (key, value) in issue.fields.iter() {
            if story_point_fields.contains_key(key) {
                story_points = larger_option_number(story_points, value.as_f64());
            }
        }

        let issue_type_name = issue.issue_type().map(|it| it.name.clone());
        let mut is_epic = false;

        if let Some(issue_type_name_ref) = &issue_type_name {
            issue_count_by_issue_type.insert(
                issue_type_name_ref.clone(),
                issue_count_by_issue_type
                    .get(issue_type_name_ref)
                    .unwrap_or(&0)
                    + 1,
            );
            if issue_type_name_ref.to_lowercase().contains("epic") {
                is_epic = true;
            } else {
                story_points_by_issue_type.insert(
                    issue_type_name_ref.clone(),
                    story_points_by_issue_type
                        .get(issue_type_name_ref)
                        .unwrap_or(&0.0)
                        + story_points.unwrap_or(0.0),
                );
                if story_points.is_none() {
                    story_points_unset_by_issue_type.insert(
                        issue_type_name_ref.clone(),
                        story_points_unset_by_issue_type
                            .get(issue_type_name_ref)
                            .unwrap_or(&0)
                            + 1,
                    );
                }
            }
        }

        if is_epic {
            printout::print_issue_line(&issue.key, issue_type_name.as_deref(), None, None);
            continue;
        }

        let status_name = issue.status().map(|s| {
            if s.name.to_lowercase().contains("review") {
                String::from("Review")
            } else {
                s.name.clone()
            }
        });

        if let Some(ref status_name_ref) = status_name {
            issue_count_by_status.insert(
                status_name_ref.clone(),
                issue_count_by_status.get(status_name_ref).unwrap_or(&0) + 1,
            );
            story_points_by_status.insert(
                status_name_ref.clone(),
                story_points_by_status.get(status_name_ref).unwrap_or(&0.0)
                    + story_points.unwrap_or(0.0),
            );
        }

        printout::print_issue_line(
            &issue.key,
            issue_type_name.as_deref(),
            status_name.as_deref(),
            story_points,
        );

        total_issue_count += 1;
        total_story_points += story_points.unwrap_or(0.0);
    }

    printout::print_stats_for_humans(
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
