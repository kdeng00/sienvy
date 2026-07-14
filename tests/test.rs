#[cfg(test)]
mod tests {
    #[test]
    fn test_dburl() {
        let result = sienvy::environment::get_db_url();
        assert_eq!(
            result.value, "postgres://myuser:password@localhost:5432/my_db",
            "DATABASE_URL does not match {:?}",
            result
        );
    }

    #[test]
    fn test_get_secret_main_key() {
        let result = sienvy::environment::get_secret_main_key();
        assert_eq!(
            result.value, "Somesupersecretpassword!!!45345435",
            "SECRET_MAIN_KEY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_service_passphrase() {
        let result = sienvy::environment::get_service_passphrase();
        assert_eq!(
            result.value, "T5OCHDHadAtuOWIqRAS7u8XHDDkzKT1Uvvw7mGMkNzKjVdlHA8xGdILf2adDHspO",
            "SERVICE_PASSPHRASE does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_secret_key() {
        let result = sienvy::environment::get_secret_key();
        assert_eq!(
            result.value, "AmIGoodEnoughForYou?",
            "SECRET_KEY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_root_directory() {
        let result = sienvy::environment::get_root_directory();
        assert_eq!(
            result.value, "/path/to/root",
            "ROOT_DIRECTORY does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_icarus_base_api_url() {
        let result = sienvy::environment::get_icarus_base_api_url();
        assert_eq!(
            result.value, "https://icarus.com",
            "ICARUS_BASE_API_URL does not match {:?}",
            result
        )
    }

    #[test]
    fn test_get_app_env() {
        let result = sienvy::environment::get_app_env();
        assert_eq!(
            result.value,
            "development",
            "{} does not match {:?}",
            sienvy::keys::APP_ENV,
            result
        )
    }

    #[test]
    fn test_get_backend_port() {
        let result = sienvy::environment::get_backend_port();
        assert_eq!(
            result.value,
            "8001",
            "{} does not match {:?}",
            sienvy::keys::BACKEND_PORT,
            result
        )
    }
    #[test]
    fn test_get_frontend_url() {
        let result = sienvy::environment::get_frontend_url();
        assert_eq!(
            result.value,
            "http://localhost:4200",
            "{} does not match {:?}",
            sienvy::keys::FRONTEND_URL,
            result
        )
    }
    #[test]
    fn test_get_rust_log() {
        let result = sienvy::environment::get_rust_log();
        assert_eq!(
            result.value,
            "debug",
            "{} does not match {:?}",
            sienvy::keys::RUST_LOG,
            result
        )
    }
    #[test]
    fn test_get_allowed_origins() {
        let result = sienvy::environment::get_allowed_origins();
        assert_eq!(
            result.value,
            "https://soaricarus.com,https://www.soaricarus.com",
            "{} does not match {:?}",
            sienvy::keys::ALLOWED_ORIGINS,
            result
        );

        assert_eq!(
            result.has_delimiter, true,
            "The {} variable has an issue finding the delimiter",
            result.key
        );

        match sienvy::utility::delimitize(&result) {
            Ok(allowed_origins) => {
                assert_eq!(
                    allowed_origins.len(),
                    2,
                    "The amount of allowed origins does not match. {} {}",
                    allowed_origins.len(),
                    2
                )
            }
            Err(err) => {
                assert!(false, "Error: {:?}", err)
            }
        }
    }

    #[test]
    fn test_get_env() {
        let keys = vec![
            (
                "RANDOM_ENV_KEY",
                "YouDon'tWantToButYouGottaChange|It'sGoingToHurtYouTryingToStayTheSame|AreYouInItOrYouInItForTheFame?|I'mTryingToFigureOutWhoLoveMeForMe",
            ),
            (
                "MODNAR_ENV_KEY",
                "FeelingTheMonsterClimbDeepserInsideOfMe|FeelingHimGnawingMyHeartAwayHungrily|I'llNeverLoseThisPain|NeverDreamOfYouAgain",
            ),
        ];

        for (key, value) in keys.iter() {
            let result = sienvy::environment::get_env(key);
            assert_eq!(
                result.value, *value,
                "{:?} does not match {:?}",
                key, result
            )
        }
    }
}
