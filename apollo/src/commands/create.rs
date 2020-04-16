use crate::commands::{Command, CreateGraph};
use crate::commands::utils;
use crate::commands::utils::get_auth;
use crate::graphql;

impl Command for CreateGraph {
    fn run(&self) {
        let auth_token = match get_auth() {
            Err(e) => {
                println!("Error authenticating: {}", e);
                return;
            }
            Ok(token) => token,
        };
        let gql_client = graphql::client::ApolloCloudClient::new(
            String::from("https://engine-staging-graphql.apollographql.com"),
            auth_token,
        );

        let accounts = gql_client.get_org_memberships();
        let accounts_pretty = format!("{:?}", accounts);
        let account = if (accounts.is_empty()) {
            println!("You are not a member of any organization");
            return;
        } else if (accounts.len() > 1) {
            let prompt_string = format!("Please choose an organization to own the graph: Options {}", accounts_pretty);
            let chosen_account = utils::get_user_input(&prompt_string).unwrap();
            if !accounts.contains(&chosen_account) {
                println!("Chose a bad account");
                return;
            }
            chosen_account
        } else {
            String::from(accounts.iter().next().unwrap())
        };
        let graph_id = utils::get_user_input("Choose a name for your graph (cannot be changed)").unwrap();
        println!("You have chosen {}. Excellent selection.", graph_id);
    }
}