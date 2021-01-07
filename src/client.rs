use reqwest::blocking;
use std::collections::HashMap;

pub fn build_api_url(key: &str) -> String {
    return format!("https://api.metalabs.io/v2/licenses/{}", key);
}

pub struct Client {
    pub auth_key: String,
}

impl Client {
    pub fn new(key: &str) -> Client {
        Client {
            auth_key: String::from(key),
        }
    }

    pub fn get_key(&self, key: &str) -> String {
        let response = blocking::Client::new()
            .get(&build_api_url(key))
            .header("Authorization", format!("Basic {}", self.auth_key))
            .send();

        let response = match response {
            Ok(res) => res,
            Err(error) => panic!("Error Occured: {:#?}", error),
        };

        let data = match response.text() {
            Ok(content) => content,
            Err(error) => panic!("Error Occured: {:#?}", error),
        };

        return data;
    }

    pub fn update_key(&self, key: &str, meta: HashMap<&str, &str>) -> String {
        let response = blocking::Client::new()
            .patch(&build_api_url(key))
            .json(&meta)
            .header("Authorization", format!("Basic {}", self.auth_key))
            .send();

        let response = match response {
            Ok(res) => res,
            Err(error) => panic!("Error Occured: {:#?}", error),
        };

        let data = match response.text() {
            Ok(content) => content,
            Err(error) => panic!("Error Occured: {:#?}", error),
        };

        return data;
    }
}