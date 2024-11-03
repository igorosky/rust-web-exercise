pub(crate) const UPLOAD_DIRECTORY: &str = "UPLOAD_DIRECTORY";
pub(crate) const UPLOAD_BUFFER_SIZE: &str = "UPLOAD_BUFFER_SIZE";
pub(crate) const MAX_BODY_SIZE: &str = "MAX_BODY_SIZE";
pub(crate) const DATABASE_URL: &str = "DATABASE_URL";
pub(crate) const STATIC_FILES_DIRECTORY: &str = "STATIC_FILES_DIRECTORY";
pub(crate) const ADDRESS: &str = "ADDRESS";

#[derive(Debug, thiserror::Error)]
#[error("Invalid environment variable {name} - {error}")]
pub(crate) struct GettingEnvVarError {
    pub(crate) name: String,
    pub(crate) error: std::env::VarError,
}


#[inline]
#[cfg(debug_assertions)]
pub(crate) fn debug_mode_initialization() {
    dotenvy::dotenv().ok();
}

#[inline]
pub(crate) fn get_env_var(name: &str) -> Result<String, GettingEnvVarError> {
    std::env::var(name)
        .map_err(|error| GettingEnvVarError { name: name.to_string(), error } )
}
