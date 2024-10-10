use std::convert::{TryFrom, TryInto};



#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings{
    pub port: u16,
    pub host: String
}

pub enum Environment {
    Local,
    Production
}

// ensure a string cast compile time fails if it does not match
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom <String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            e => Err(format!("{} is not a supported environment. Use 'local' or 'production' ", e))
        }
    }
}

// read application settings from a flat file named configuration:
pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let base_path = std::env::current_dir().expect("Failed to find current directory");
    let config_dir = base_path.join("configuration");

    // check environment for dev/prod
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base")))
        .add_source(config::File::from(config_dir.join(environment.as_str())))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
        self.username, self.password, self.host, self.port, self.database_name)
    }
}