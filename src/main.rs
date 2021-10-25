extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};
// use goji::issues::{IssueType};
use std::env;

fn main() -> Result<(), &'static str> {
    drop(env_logger::init());
    if let (Ok(host), Ok(user), Ok(pass)) =
    (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
    )
    {
        let query = env::args().nth(1).unwrap_or("".to_owned());

        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    // println!("{:#?}", issue);
                    match issue.project() {
                        None => println!("Project unkown!"),
                        Some(project) => println!("{}", project.key.as_str()),
                    };
                    println!("{0}\t{1}", issue.key, issue.summary().unwrap_or("unset".to_owned()));
                    // println!("{0}", issue.field("Story points").unwrap_or(Ok("unset".to_owned())).unwrap_or("error".to_owned()));
                    match issue.issue_type() {
                        None => {
                            println!("Unknown type!");
                        }
                        Some(issue_type) => {
                            println!("{0}", issue_type.name)
                        }
                    }
                    // for (key, value) in issue.fields.iter() {
                    //     println!("{0} {1:#?}", key, value);
                    // }
                    println!("-----");
                }
            }
            Err(err) => panic!("{:#?}", err),
        }
        Ok(())
    } else {
        return Err("Env variables JIRA_HOST, JIRA_USER & JIRA_PASS need to be set!")
    }
}
