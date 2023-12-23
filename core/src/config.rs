use crate::Error;
use log::{info, warn};
use log4rs::append::console::ConsoleAppender;
pub struct Config{}

pub enum Mode {
    Test,
    Development,
    Production
}

impl From<String> for Mode {
    fn from(val: std::string::String) -> Self {
        match val.as_str() {
            "test" => Mode::Test,
            "dev" => Mode::Development,
            "prod" => Mode::Production,
            _ => Mode::Development
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = self.into();
        write!(f, "{}", str)
    }
}

impl Into<String> for &Mode {
    fn into(self) -> String {
        match self {
            Mode::Test => "test".into(),
            Mode::Production => "prod".into(),
            Mode::Development => "dev".into()
        }
    }
}

#[derive(Default)]
pub struct ConfigArgs {
    mode: Option<Mode>
}

impl ConfigArgs {
    pub fn set_mode(mut self, mode: Mode) -> Self {
        self.mode = Option::Some(mode);
        self
    }
}

impl Config {
    /// Initialise configuration
    pub fn init(args: ConfigArgs) -> Result<(), Error> {
        let _ = Self::init_logging();

        if let Some(mode) = &args.mode {
            let str_mode: String = mode.into();
            std::env::set_var("MODE", str_mode);
        }

        // Get the current mode.
        let mode = Self::get_mode();
        
        info!(target: "signuis::config", "Executing from {:?}.", std::env::current_dir().unwrap().as_os_str().to_os_string());
        info!(target: "signuis::config", "Configuring for {} environment...", mode);

        let env_file = match mode {
            Mode::Test => "test.env",
            Mode::Development => "dev.env",
            Mode::Production => ".env"
        };

        // Load file
        if !Self::load_env_file(env_file) && env_file != ".env" {
            warn!(target: "signuis::config", "Falling back to .env...");
            Self::load_env_file(".env");
        }

        Ok(())
    }

    fn init_logging() -> Result<(), Error> {
        let stdout = ConsoleAppender::builder().build();
    
        let config = log4rs::config::Config::builder()
            .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
            .build(log4rs::config::Root::builder().appender("stdout").build(log::LevelFilter::Info))
            .unwrap();
    
        log4rs::init_config(config)?;
        Ok(())
    }

    fn load_env_file(env_file: &str) -> bool {
        match std::fs::metadata(env_file) {
            Ok(_) => {
                info!(target: "signuis::config", "Loading environment file \"{}\".", env_file);
                dotenv::from_filename(env_file).ok();
                return true;
            },

            Err(_) => {
                warn!(target: "signuis::config", "No environment file \"{}\" existing!", env_file);
                return false;
            }
        };     
    }

    pub fn get_mode() -> Mode {
        if let Some(val) = std::env::var("MODE").ok() {
            return val.into()
        } 
        return Mode::Production;
    }

    pub fn try_get_database_url() -> Result<String, Error> {
        std::env::var("DATABASE_URL").map_err(|_| Error::missing_env("DATABASE_URL"))
    }
}