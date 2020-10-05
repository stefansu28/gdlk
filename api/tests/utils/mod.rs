//! A helper module to hold utilities that are used across tests. This file
//! DOES NOT container any of its own tests.

pub mod factories;

use crate::utils::factories::*;
use diesel::PgConnection;
use diesel_factories::Factory;
use gdlk_api::{
    models,
    server::{create_gql_schema, GqlSchema},
    util::{self, PooledConnection},
    views::RequestContext,
};
use juniper::{ExecutionError, InputValue, Variables};
use serde::Serialize;
use std::collections::HashMap;

/// Convert a serializable value into a JSON value.
#[allow(dead_code)] // Not all test crates use this
pub fn to_json<T: Serialize>(input: T) -> serde_json::Value {
    let serialized: String = serde_json::to_string(&input).unwrap();
    serde_json::from_str(&serialized).unwrap()
}

pub struct ContextBuilder {
    db_conn: PooledConnection,
    user_provider: Option<models::UserProvider>,
    user: Option<models::User>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        let db_conn = util::init_test_db_conn_pool().unwrap().get().unwrap();
        Self {
            db_conn,
            user_provider: None,
            user: None,
        }
    }

    pub fn db_conn(&self) -> &PgConnection {
        &self.db_conn
    }

    #[allow(dead_code)] // Not all test crates use this
    pub fn set_user_provider(&mut self, user_provider: models::UserProvider) {
        self.user_provider = Some(user_provider);
    }

    #[allow(dead_code)] // Not all test crates use this
    pub fn log_in(&mut self, roles: &[models::RoleType]) -> models::User {
        let conn = self.db_conn();
        let user = UserFactory::default().username("user1").insert(conn);

        // Create a bogus user_provider for this user. We're not trying to test
        // the OpenID logic here, so this is fine.
        let user_provider = UserProviderFactory::default()
            .sub(&user.id.to_string()) // guarantees uniqueness
            .user(Some(&user))
            .insert(conn);

        // Insert one row into user_roles for each requested row
        user.add_roles_x(conn, roles).unwrap();

        self.user_provider = Some(user_provider);
        self.user = Some(user.clone());
        user
    }

    #[allow(dead_code)] // Not all test crates use this
    pub fn build(self) -> RequestContext {
        RequestContext::load_context(
            self.db_conn,
            self.user_provider.map(|up| up.id),
        )
        .unwrap()
    }
}

/// Helper type for setting up and executing test GraphQL queries
#[allow(dead_code)] // Not all test crates use this
pub struct QueryRunner {
    schema: GqlSchema,
    context: RequestContext,
}

impl QueryRunner {
    #[allow(dead_code)] // Not all test crates use this
    pub fn new(context_builder: ContextBuilder) -> Self {
        Self {
            schema: create_gql_schema(),
            context: context_builder.build(),
        }
    }

    #[allow(dead_code)] // Not all test crates use this
    pub fn db_conn(&self) -> &PgConnection {
        &self.context.db_conn
    }

    #[allow(dead_code)] // Not all test crates use this
    pub fn query<'a>(
        &'a self,
        query: &'a str,
        vars: HashMap<&str, InputValue>,
    ) -> (serde_json::Value, Vec<serde_json::Value>) {
        // Map &strs to Strings
        let converted_vars = vars
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Variables>();

        let (data, errors): (juniper::Value<_>, Vec<ExecutionError<_>>) =
            juniper::execute(
                query,
                None,
                &self.schema,
                &converted_vars,
                &self.context,
            )
            .unwrap();

        // Map the output data to JSON, for easier comparison
        (to_json(data), errors.into_iter().map(to_json).collect())
    }
}
