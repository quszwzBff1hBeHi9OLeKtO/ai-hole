use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) default_random_words_count: usize,
    pub(crate) selectors: SelectorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_random_words_count: 5,
            selectors: SelectorConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct SelectorConfig {
    pub(crate) html: HTMLSelectorConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct HTMLSelectorConfig {
    pub(crate) randomized: Vec<String>,
    pub(crate) removed: Vec<String>,
}
