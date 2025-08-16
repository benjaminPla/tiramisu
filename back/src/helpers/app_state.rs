use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct EnvVars {
    pub db_conn_string: String,
    pub db_max_conn: u32,
    pub jwt_secret: String,
    pub timeout_duration: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub env_vars: EnvVars,
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new() -> Result<Arc<Self>, String> {
        let env_vars = Self::load_env_vars()?;
        let db_pool = Self::create_db_pool(&env_vars).await?;
        Ok(Arc::new(Self { env_vars, db_pool }))
    }

    fn load_env_vars() -> Result<EnvVars, String> {
        let db_conn_string = Self::get_env_var("DB_CONN_STRING")?;
        let db_max_conn = Self::get_env_var_parse::<u32>("DB_MAX_CONN")?;
        let jwt_secret = Self::get_env_var("JWT_SECRET")?;
        let timeout_duration = Self::get_env_var_parse::<u64>("TIMEOUT_DURATION")?;
        Ok(EnvVars {
            db_conn_string,
            db_max_conn,
            jwt_secret,
            timeout_duration,
        })
    }

    fn get_env_var(name: &str) -> Result<String, String> {
        env::var(name).map_err(|_| format!("Missing env var: {}", name))
    }

    fn get_env_var_parse<T: std::str::FromStr>(name: &str) -> Result<T, String>
    where
        T::Err: std::fmt::Display,
    {
        let value = Self::get_env_var(name)?;
        value
            .parse()
            .map_err(|e| format!("Failed to parse {} env var: {}", name, e))
    }

    async fn create_db_pool(env: &EnvVars) -> Result<PgPool, String> {
        PgPoolOptions::new()
            .max_connections(env.db_max_conn)
            .connect(&env.db_conn_string)
            .await
            .map_err(|e| format!("Error connecting to database: {}", e))
    }
}
