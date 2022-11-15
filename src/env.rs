use once_cell::sync::Lazy;

pub static API_ENDPOINT: Lazy<String> = Lazy::new(|| std::env::var("API_ENDPOINT").expect("API_ENDPOINT must be set"));