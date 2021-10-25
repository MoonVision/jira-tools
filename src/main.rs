use std::borrow::Borrow;
extern crate env_logger;
extern crate goji;
use std::collections::{HashMap, HashSet};
use goji::{Credentials, Jira};
use goji::issues::{IssueType};
use std::env;
use std::io::{Error, ErrorKind};
use url::form_urlencoded;
use serde_derive::Deserialize;

fn create_error(error: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(Error::new(
        ErrorKind::Other,
        error,
    )))
}

/*
{
    "clauseNames": [
        "issueLink"
    ],
    "custom": false,
    "id": "issuelinks",
    "key": "issuelinks",
    "name": "Linked Issues",
    "navigable": true,
    "orderable": true,
    "schema": {
        "items": "issuelinks",
        "system": "issuelinks",
        "type": "array"
    },
    "searchable": true
}
 */
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
            .send().await?
            .json::<Vec<JiraField>>().await?;
        // let story_point_fields: Vec<&JiraField> = resp.iter()
        //     .filter(|jira_field| jira_field.name.to_lowercase().contains("story point"))
        //     .collect();
        //
        // let mut story_point_field_ids: HashSet<String> = HashSet::new();
        // resp.iter()
        //     .filter(|jira_field| jira_field.name.to_lowercase().contains("story point"))
        //     .for_each(|jira_field| {
        //         story_point_field_ids.insert(jira_field.id.clone());
        //     });

        // story_point_fields.iter().for_each(|item| {
        //     println!("Field name:\t{}", item.name);
        //     println!("Field id:\t{}", item.id);
        //     println!("-----");
        // });

        let mut story_point_fields: HashMap<String, &JiraField> = HashMap::new();
        resp.iter()
            .filter(|jira_field| jira_field.name.to_lowercase().contains("story point"))
            .for_each(|jira_field| {
                story_point_fields.insert(jira_field.id.clone(), jira_field);
            });

        // story_point_field_ids.into_iter().for_each(|item| println!("Field id:\t{}\n-----", item));
        // println!("story_point_field_ids {:#?}", story_point_field_ids);

        let query = env::args().nth(1).unwrap_or("".to_owned());

        let jira = Jira::new(jira_host, Credentials::Basic(jira_user, jira_pass)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!("Key:\t\t\t{0}", issue.key);
                    println!("Summary:\t\t{0}", issue.summary().unwrap_or("unset".to_owned()));
                    match issue.issue_type() {
                        None => {
                            println!("Type:\t\t\tUnknown!");
                        }
                        Some(issue_type) => {
                            println!("Type:\t\t\t{0}", issue_type.name)
                        }
                    }
                    // println!("- - -");
                    for (key, value) in issue.fields.iter() {
                        if story_point_fields.contains_key(key) {
                            println!("{}:\t{}", story_point_fields.get(key).unwrap().name, value);
                        }
                        // issue.fields.get(key).map(|v| println!("{0} {1:#?}", key, value));
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
