

#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

// read application settings from a flat file named configuration:
pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let base_path = std::env::current_dir().expect("Failed to find current directory");
    let settings = config::Config::builder()
    .add_source(config::File::from(base_path.join("src/configuration.yaml")))
    .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
        self.username, self.password, self.host, self.port, self.database_name)
    }
}