extern crate env_logger;
extern crate goji;
use goji::issues::IssueType;
use goji::{Credentials, Jira};
use serde_derive::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{Error, ErrorKind};
use url::form_urlencoded;

fn create_error(error: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, error)))
}

#[derive(Deserialize, Debug)]
struct JiraField {
    custom: bool,
    id: String,
    key: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    drop(env_logger::init());
    if let (Ok(jira_host), Ok(jira_user), Ok(jira_pass)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
    ) {
        let field_endpoint = format!("{}/rest/api/3/field", jira_host);
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

        let query = env::args().nth(1).unwrap_or("".to_owned());

        let jira = Jira::new(jira_host, Credentials::Basic(jira_user, jira_pass)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!("Key:\t\t\t{0}", issue.key);
                    println!(
                        "Summary:\t\t{0}",
                        issue.summary().unwrap_or("unset".to_owned())
                    );
                    match issue.issue_type() {
                        None => {
                            println!("Type:\t\t\tUnknown!");
                        }
                        Some(issue_type) => {
                            println!("Type:\t\t\t{0}", issue_type.name)
                        }
                    }
                    for (key, value) in issue.fields.iter() {
                        if story_point_fields.contains_key(key) {
                            println!("{}:\t{}", story_point_fields.get(key).unwrap().name, value);
                        }
                    }
                    println!("-----");
                }
            }
            Err(err) => panic!("{:#?}", err),
        }
        Ok(())
    } else {
        create_error("Env variables JIRA_HOST, JIRA_USER & JIRA_PASS need to be set!")
    }
}
