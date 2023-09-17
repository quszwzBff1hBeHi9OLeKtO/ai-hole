use crate::config::Config;
use crate::stats::Stats;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) struct AppState {
    wordlist: Vec<String>,
    pub(crate) config: Config,
    pub(crate) stats: Stats,
}
impl AppState {
    pub fn new() -> Self {
        let mut words = String::new();
        File::open("wordlist.txt")
            .expect("wordlist.txt file needed")
            .read_to_string(&mut words)
            .expect("failed to read wordlist.txt");
        let wordlist = words
            .trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let config_path = Path::new("config.yaml");

        let config = match config_path.exists() {
            true => {
                let mut config = String::new();
                File::open(config_path)
                    .expect("failed to open file")
                    .read_to_string(&mut config)
                    .expect("failed to read config");
                let config = serde_yaml::from_str(&config).expect("malformed config");
                tracing::info!("Using config.yaml for config");
                config
            }
            false => {
                tracing::info!("No configuration provided, using default config!");
                Config::default()
            }
        };
        tracing::info!("Using config: {config:#?}");

        Self {
            wordlist,
            config,
            stats: Stats::new(),
        }
    }
    pub(crate) fn generate_word_sequence(&self, count: usize) -> String {
        let words = self
            .wordlist
            .choose_multiple(&mut rand::thread_rng(), count)
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        words.join(" ")
    }
}
