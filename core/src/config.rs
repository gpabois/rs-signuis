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

impl Into<String> for Mode {
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

impl Config {
    /// Initialise configuration
    pub fn init(args: ConfigArgs) {
        if let Some(mode) = args.mode {
            let str_mode: String = mode.into();
            std::env::set_var("MODE", str_mode);
        }

        // Get the current mode.
        let mode = Self::get_mode();

        let env_file = match mode {
            Mode::Test => "test.env",
            Mode::Development => "dev.env",
            Mode::Production => ".env"
        };

        // Load file
        dotenv::from_filename(env_file).ok();
    }

    pub fn get_mode() -> Mode {
        if let Some(val) = std::env::var("MODE").ok() {
            return val.into()
        } 
        return Mode::Production;
    }

    pub fn get_database_url() -> Option<String> {
        std::env::var("DATABASE_URL").ok()
    }
}