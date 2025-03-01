pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,

    pub srv_host: String,
    pub srv_port: u16,
}

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();

        Self {
            db_host: std::env::var("DB_HOST").expect("DB_HOST must be set"),
            db_port: std::env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse()
                .expect("DB_PORT must be a number"),
            db_user: std::env::var("DB_USER").expect("DB_USER must be set"),
            db_password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
            db_name: std::env::var("DB_NAME").expect("DB_NAME must be set"),
            srv_host: std::env::var("SRV_HOST").expect("SRV_HOST must be set"),
            srv_port: std::env::var("SRV_PORT")
                .expect("SRV_PORT must be set")
                .parse()
                .expect("SRV_PORT must be a number"),
        }
    }

    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }
}
