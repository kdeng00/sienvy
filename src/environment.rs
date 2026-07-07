pub fn get_db_url() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::DB_URL;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_secret_main_key() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::SECRET_MAIN_KEY;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_service_passphrase() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::SERVICE_PASSPHRASE;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_secret_key() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::SECRET_KEY;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_root_directory() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::ROOT_DIRECTORY;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_icarus_base_api_url() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::ICARUS_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_icarus_auth_base_api_url() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::ICARUS_AUTH_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_app_env() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::APP_ENV;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_backend_port() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::BACKEND_PORT;
    let value = std::env::var(key).expect(key);
    crate::init_envvar(key, &value)
}

pub fn get_frontend_url() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::FRONTEND_URL;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_rust_log() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::RUST_LOG;
    let value = std::env::var(key).expect(key);

    crate::init_envvar(key, &value)
}

pub fn get_allowed_origins() -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::keys::ALLOWED_ORIGINS;
    let value = std::env::var(key).expect(key);

    let mut envvar = crate::init_envvar(key, &value);
    crate::init_delimiter(&mut envvar, ',');

    envvar
}

/// Get environment not specified in the code
pub fn get_env(environment: &str) -> crate::EnvVar {
    dotenvy::dotenv().ok();
    let my_error = format!("{environment} {}", crate::keys::error::GENERAL_ERROR);
    let value = std::env::var(environment).expect(&my_error);

    crate::init_envvar(environment, &value)
}
