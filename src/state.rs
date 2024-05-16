use crate::errors::AppError;
use crate::models::Post;
use axum::extract::FromRef;
//use chrono::{DateTime, Utc};
use graphql_client::GraphQLQuery;
use indexmap::IndexMap;
use leptos::logging::log;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use parking_lot::RwLock;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

// Generics
type URI = String;
type DateTime = chrono::DateTime<chrono::Utc>;
type HTML = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/github_schema.graphql",
    query_path = "graphql/discussions_query.graphql",
    variables_derives = "Clone, Debug",
    response_derives = "Clone, Debug"
)]
pub struct DiscussionsQuery;

//query {
//  repository(owner: "polarmutex", name:"website") {
//    discussionCategories(first:10){
//        edges{
//          node {
//            id
//            name
//          }
//        }}}}
// https://docs.github.com/en/graphql/overview/explorer
// https://docs.github.com/en/graphql/guides/using-pagination-in-the-graphql-api

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Posts {
    pub posts: IndexMap<String, Post>,
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

impl Posts {
    pub async fn fetch_posts_from_github(&mut self) -> Result<(), AppError> {
        log!("Returning empty posts");
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
            .build()?;

        let mut variables = discussions_query::Variables {
            owner: "polarmutex".to_string(),
            name: "website".to_string(),
            //page_size: 5,
            //before: None,
        };

        let response: octocrab::Result<graphql_client::Response<discussions_query::ResponseData>> =
            octocrab
                .graphql(&DiscussionsQuery::build_query(variables.clone()))
                .await;

        match response {
            Ok(response) => {
                println!("{:?}", response);
                let issues = &response
                    .data
                    .as_ref()
                    .unwrap()
                    .repository
                    .as_ref()
                    .unwrap()
                    .discussions;
                print_issues(issues);
            }
            Err(error) => {
                println!("{error:#?}");
            }
        }

        Ok(())
    }
}
fn print_issues(issues: &discussions_query::DiscussionsQueryRepositoryDiscussions) {
    for issue in issues.nodes.as_ref().unwrap().iter().flatten() {
        println!("{:?}", issue);
    }
}

#[derive(Clone, Debug, Default)]
pub struct PostsContainer(pub Arc<RwLock<Posts>>);

impl PostsContainer {
    pub async fn new_with_posts() -> Result<Self, AppError> {
        let mut posts = Posts::default();
        posts.fetch_posts_from_github().await?;
        posts
            .posts
            .sort_unstable_by(|_a, b, _c, d| d.created_at.partial_cmp(&b.created_at).unwrap());

        let container = PostsContainer(Arc::new(RwLock::new(posts)));
        Ok(container)
    }
}

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub posts: PostsContainer,
    pub routes: Vec<RouteListing>,
}

impl AppState {
    pub async fn new_with_posts(
        leptos_options: LeptosOptions,
        routes: Vec<RouteListing>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            leptos_options,
            posts: PostsContainer::new_with_posts().await?,
            routes,
        })
    }
}
