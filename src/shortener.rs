use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use rand::seq::SliceRandom;

lazy_static! {
    pub static ref INSTANCE: Mutex<URLShortener> = Mutex::new(URLShortener::new());
}

pub struct URLShortener {
    urls: HashMap<String, String>,
    base_url: String,
}

impl URLShortener {
    fn new() -> Self {
        URLShortener {
            urls: HashMap::new(),
            base_url: match std::env::var("BASE_URL") {
                Ok(val) => val,
                _ => String::from("http://localhost:8080/"),
            },
        }
    }

    fn generate_id(&self) -> String {
        const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut id = String::new();
        for _ in 0..8 {
            let mut rng = rand::thread_rng();

            if let Some(&random_char) = CHARS.chars().collect::<Vec<char>>().choose(&mut rng) {
                id.push_str(random_char.to_string().as_str());
            }
        }
        id
    }

    pub fn shorten(&mut self, url: &String) -> String {
        let id = self.generate_id();
        self.urls.insert(id.clone(), url.to_string());
        format!("{}{}", self.base_url, id)
    }

    pub fn expand(&self, url: &mut String) -> String {
        let key = url.replace(&self.base_url, "");

        match self.urls.get(&key) {
            Some(u) => u.to_string(),
            None => String::new(),
        }
    }
}

pub fn instance() -> std::sync::MutexGuard<'static, URLShortener> {
    INSTANCE.lock().unwrap()
}
