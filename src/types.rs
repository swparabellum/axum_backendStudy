use crate::jwt::JwtSigner;
use axum::extract::FromRef;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;

pub(crate) mod config;

#[derive(Clone)]
pub(crate) struct AppState {
    pub db_conn_pool: DatabaseConnectionPool,
    pub jwt_signer: Arc<JwtSigner>,
}

#[derive(Clone)]
pub(crate) struct DatabaseConnectionPool(
    pub Arc<bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
);

impl FromRef<AppState> for DatabaseConnectionPool {
    fn from_ref(input: &AppState) -> Self {
        input.db_conn_pool.clone()
    }
}

impl FromRef<AppState> for Arc<JwtSigner> {
    fn from_ref(input: &AppState) -> Self {
        input.jwt_signer.clone()
    }
}
