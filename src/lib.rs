use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use graphql_client::{
    GraphQLQuery,
    reqwest::post_graphql_blocking as post_graphql,
};


pub struct Api {
    url: String,
    key: String,
    client: Client,
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/query/list_all_pages.graphql",
    response_derives = "Debug"
)]
pub struct ListAllPages;


impl Api {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            key: key.clone(),
            client:
                Client::builder()
                    .user_agent("wikijs-rs/0.1.0")
                    .default_headers(
                        std::iter::once((
                            AUTHORIZATION,
                            HeaderValue::from_str(&format!("Bearer {}", key))
                                        .unwrap()
                            ))
                        .collect(),
                     )
                    .build()
                    .unwrap(),
        }
    }

    pub fn list_all_pages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response_body = post_graphql::<ListAllPages, _>(
            &self.client,
            &format!("{}/graphql", self.url),
            list_all_pages::Variables {}
        )?;

        println!("{:#?}", response_body);
        Ok(())
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
