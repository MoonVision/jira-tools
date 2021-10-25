use std::borrow::Borrow;
// extern crate env_logger;
// extern crate goji;
use std::collections::HashMap;
// use goji::{Credentials, Jira};
// use goji::issues::{IssueType};
use std::env;
use std::io::{Error, ErrorKind};
use url::form_urlencoded;

fn create_error(error: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(Error::new(
        ErrorKind::Other,
        error,
    )))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // drop(env_logger::init());
    if let (Ok(jira_host), Ok(jira_user), Ok(jira_pass)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
    ) {
        let sprint_endpoint = format!("{}/rest/api/3/search?jql=Sprint+%3D+84", jira_host);
        let resp = reqwest::Client::new()
            .get(sprint_endpoint)
            .basic_auth(jira_user.clone(), Some(jira_pass.clone()))
            .send().await?
            .text().await?;
        println!("{:#?}", resp);

        let field_endpoint = format!("{}/rest/api/3/field", jira_host);
        let resp = reqwest::Client::new()
            .get(field_endpoint)
            .basic_auth(jira_user.clone(), Some(jira_pass.clone()))
            .send().await?
            .text().await?;
        println!("{:#?}", resp);
        // let query = env::args().nth(1).unwrap_or("".to_owned());
        //
        // let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();
        //
        // match jira.search().iter(query, &Default::default()) {
        //     Ok(results) => {
        //         for issue in results {
        //             // println!("{:#?}", issue);
        //             match issue.project() {
        //                 None => println!("Project unkown!"),
        //                 Some(project) => println!("{}", project.key.as_str()),
        //             };
        //             println!("{0}\t{1}", issue.key, issue.summary().unwrap_or("unset".to_owned()));
        //             // println!("{0}", issue.field("Story points").unwrap_or(Ok("unset".to_owned())).unwrap_or("error".to_owned()));
        //             match issue.issue_type() {
        //                 None => {
        //                     println!("Unknown type!");
        //                 }
        //                 Some(issue_type) => {
        //                     println!("{0}", issue_type.name)
        //                 }
        //             }
        //             // println!("- - -");
        //             for (key, value) in issue.fields.iter() {
        //                 issue.fields.get(key).map(|v| println!("{0} {1:#?}", key, value));
        //
        //             }
        //             println!("-----");
        //         }
        //     }
        //     Err(err) => panic!("{:#?}", err),
        // }
        // Ok(())
        Ok(())
    } else {
        create_error("Env variables JIRA_HOST, JIRA_USER & JIRA_PASS need to be set!")
    }
}
