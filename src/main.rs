mod types;
mod jwt;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::State;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use jwt_authorizer::{Authorizer, IntoLayer, JwtAuthorizer, JwtClaims, Validation};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use crate::jwt::JwtSigner;
use crate::types::{AppState, DatabaseConnectionPool};
use crate::types::config::read_config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Initialize tracing_subscriber
    tracing_subscriber::fmt::init();

    let config = read_config("./config.toml").await;

    let diesel_conn_mgr =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(config.database.url);

    let pool = bb8::Pool::builder().build(diesel_conn_mgr).await.unwrap();

    let state = AppState {
        db_conn_pool: DatabaseConnectionPool(Arc::new(pool)),
        jwt_signer: Arc::new(JwtSigner::new(
            config.jwt_key.key.clone(),
            config.jwt_key.kid,
            "axum-example".to_string(),
            "axum-example".to_string(),
        )),
    };

    let app = Router::new()
        .route("/",get(index))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("server on {}",listener.local_addr()?);
    axum::serve(listener,app).await?;

    Ok(())
}

async fn index() -> Html<&'static str>{
    let html_content = include_str!("../front/aboutMe.html");
    Html(html_content)
}