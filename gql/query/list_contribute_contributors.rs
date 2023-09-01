#![allow(clippy::all, warnings)]
pub struct ListContributeContributors;
pub mod list_contribute_contributors {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListContributeContributors";
    pub const QUERY : & str = "query ListContributeContributors {\n  contribute {\n    contributors {\n      id\n      source\n      name\n      joined\n      website\n      twitter\n      avatar\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type Date = super::Date;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub contribute: Option<ListContributeContributorsContribute>,
    }
    #[derive(Deserialize)]
    pub struct ListContributeContributorsContribute {
        pub contributors: Option<Vec<Option<ListContributeContributorsContributeContributors>>>,
    }
    #[derive(Deserialize)]
    pub struct ListContributeContributorsContributeContributors {
        pub id: String,
        pub source: String,
        pub name: String,
        pub joined: Date,
        pub website: Option<String>,
        pub twitter: Option<String>,
        pub avatar: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for ListContributeContributors {
    type Variables = list_contribute_contributors::Variables;
    type ResponseData = list_contribute_contributors::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: list_contribute_contributors::QUERY,
            operation_name: list_contribute_contributors::OPERATION_NAME,
        }
    }
}
