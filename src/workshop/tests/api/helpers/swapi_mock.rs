use std::{ops::Not, time::Duration};

use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use workshop::swapi::Person;

pub struct SwapiMock {
    server: MockServer,
}

impl SwapiMock {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    pub async fn mock_people_query(&self, name: &str, response_body: Value) {
        Mock::given(method("GET"))
            .and(path("/api/people/"))
            .and(query_param("search", name))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("Content-type", "application/json")
                    .set_body_json(&response_body),
            )
            .mount(&self.server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/people/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("Content-type", "application/json")
                    .set_body_json(&empty_query_result()),
            )
            .mount(&self.server)
            .await;
    }

    pub async fn mock_people_query_with_delay(
        &self,
        name: &str,
        response_body: Value,
        delay: Duration,
    ) {
        let mut full_response = ResponseTemplate::new(200)
            .insert_header("Content-type", "application/json")
            .set_body_json(&response_body);
        let mut empty_response = ResponseTemplate::new(200)
            .insert_header("Content-type", "application/json")
            .set_body_json(&empty_query_result());

        if delay.is_zero().not() {
            [full_response, empty_response] =
                [full_response, empty_response].map(|response| response.set_delay(delay));
        }

        Mock::given(method("GET"))
            .and(path("/api/people/"))
            .and(query_param("search", name))
            .respond_with(full_response)
            .mount(&self.server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/people/"))
            .respond_with(empty_response)
            .mount(&self.server)
            .await;
    }

    pub fn uri(&self) -> String {
        self.server.uri()
    }
}

pub fn person_query_result(person: &Person) -> Value {
    json!({
        "count": 1,
        "next": null,
        "previous": null,
        "results": [{
            "name": person.name,
            "height": person.height.to_string()
        }],
    })
}

pub fn empty_query_result() -> Value {
    json!({
        "count": 0,
        "next": null,
        "previous": null,
        "results": [],
    })
}
