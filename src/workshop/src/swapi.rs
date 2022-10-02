use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub height: String,
    // pub mass: String,
    // pub hair_color: String,
    // pub eye_color: String,
    // pub birth_year: String,
    // pub gender: String,
    // pub homeworld: String,
    // pub films: Vec<String>,
    // pub species: Vec<String>,
    // pub vehicles: Vec<String>,
    // pub starships: Vec<String>,
    // created: "2014-12-09T13:50:51.644000Z",
    // edited: "2014-12-20T21:17:56.891000Z",
    // url: "https://swapi.dev/api/people/1/"
}

#[derive(Debug, Default, Clone)]
pub struct SwapiClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Response<T> {
    pub count: u32,
    pub results: Vec<T>,
}

impl SwapiClient {
    pub fn new(base_url: String, timeout: Duration) -> Result<Self, reqwest::Error> {
        let client = Client::builder().timeout(timeout).build()?;

        Ok(Self { client, base_url })
    }

    pub async fn people_by_name(&self, name: &str) -> Result<Vec<Person>, reqwest::Error> {
        let response: Response<Person> = self
            .client
            .get(format!("{}/api/people/?search={name}", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        Ok(response.results)
    }
}
