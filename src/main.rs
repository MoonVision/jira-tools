extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};
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
                    println!("{:#?}", issue);
                }
            }
            Err(err) => panic!("{:#?}", err),
        }
        Ok(())
    } else {
        return Err("Env variables JIRA_HOST, JIRA_USER & JIRA_PASS need to be set!")
    }
}
