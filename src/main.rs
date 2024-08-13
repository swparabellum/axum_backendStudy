mod types;
mod jwt;
mod schema;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::State;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router, serve};
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
use tower_http::services::ServeDir;
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
        .nest_service("/", ServeDir::new("front"))
        .route("/index",get(index))
        .route("/signup.html", get(signup))
        .nest("/api",
              Router::new()
        .route("/login", post(login)))
            .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("server on {}",listener.local_addr()?);
    axum::serve(listener,app).await?;

    Ok(())
}


async fn index() -> Html<&'static str>{
    let html_content = include_str!("../front/noNameWeb.html");
    Html(html_content)
}

async fn signup() -> Html<&'static str>{
    let html_content = include_str!("../front/signup.html");
    Html(html_content)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserCredential {
    email: String,
    password: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct Token {
    pub token: String,
    pub exp: u64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

async fn login(
    State(DatabaseConnectionPool(db_pool)): State<DatabaseConnectionPool>,
    State(signer) : State<Arc<JwtSigner>>,
    Json(user): Json<UserCredential>,
) -> Result<Json<Token>,()>{

    let mut conn = db_pool.as_ref().get().await.unwrap();
    let results = schema::users::table
        .filter(schema::users::email.eq(user.email.clone()))
        .limit(1)
        .select(User::as_select())
        .load(&mut conn)
        .await
        .unwrap();

    let user = results.get(0);

    match user {
        Some(u) => {
            let (token, exp) = signer.sign(u.id, Duration::from_secs(3600)).unwrap();

            Ok(Json(Token { token, exp }))
        }
        None => Err(()), //로그인 fail
    }
}