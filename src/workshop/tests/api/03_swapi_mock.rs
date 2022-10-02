//! You are able to retrieve the height from Swapi, great!
//! However a docker image of the service you are working with
//! is not always available.
//! In these cases you can write a mock, i.e. a piece of software
//! that mimics like the service you are calling.
//! Let's practice writing a mock using the
//! [wiremock](https://docs.rs/wiremock/) library!

use serde_json::json;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use {
    std::time::Duration,
    workshop::swapi::{Person, SwapiClient},
};

/// 💡 This test should pass even if you stop the swapi container!
#[tokio::test]
async fn retrieve_luke_height_from_swapi_mock() {
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };

    // Start a [`MockServer`](https://docs.rs/wiremock/0.5.14/wiremock/struct.MockServer.html)
    // and mock the GET response you get in the `SwapiClient`.
    // You should return the response we have seen in the previous exercise
    // when looking for Luke: a 200 status code and Luke's name and height
    // in the `results` of the body.
    let mock_server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/people/")).respond_with(ResponseTemplate::new(200).insert_header("Content-type", "application/json")
        .set_body_json(json!({"count":1,"next":null,"previous":null,"results":[{"name":"Luke Skywalker","height":"172","mass":"77","hair_color":"blond","skin_color":"fair","eye_color":"blue","birth_year":"19BBY","gender":"male","homeworld":"https://swapi.dev/api/planets/1/","films":["https://swapi.dev/api/films/1/","https://swapi.dev/api/films/2/","https://swapi.dev/api/films/3/","https://swapi.dev/api/films/6/"],"species":[],"vehicles":["https://swapi.dev/api/vehicles/14/","https://swapi.dev/api/vehicles/30/"],"starships":["https://swapi.dev/api/starships/12/","https://swapi.dev/api/starships/22/"],"created":"2014-12-09T13:50:51.644000Z","edited":"2014-12-20T21:17:56.891000Z","url":"https://swapi.dev/api/people/1/"}]}
        ))).mount(&mock_server).await;

    // Use the [uri](https://docs.rs/wiremock/0.5.14/wiremock/struct.MockServer.html#method.uri)
    // method to retrieve the base url.
    let base_url = mock_server.uri();
    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let people: Vec<Person> = swapi_client.people_by_name(&luke.name).await.unwrap();
    assert_eq!(people, vec![luke])
}

/// Spock is not a Star Wars character, so the `people_by_name` function
/// should return an empty vector.
///
/// ## Hint 💡
/// Don't worry too much about copy pasting from the previous test.
/// We are going to clean in the next exercises.
#[tokio::test]
async fn spock_is_not_found_from_swapi_mock() {
    let spock = "Spock";
    // Start a `MockServer` and mock the GET response you get in the `SwapiClient`.
    // You should return the response we have seen in the previous exercise
    // when looking for Spock: a 200 status code and an empty `results` in the body.
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/people/"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("Content-type", "application/json")
                .set_body_json(json!({"count": 0, "results": []})),
        )
        .mount(&mock_server)
        .await;

    let base_url = mock_server.uri();

    // You can ignore the timeout for this exercise.
    let timeout = Duration::from_secs(2);
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let people: Vec<Person> = swapi_client.people_by_name(spock).await.unwrap();
    assert!(people.is_empty());
}

/// Mocks allow to simulate edge cases of your dependencies.
/// For example, what happens if Swapi doesn't respond?
/// Does your application return the right error?
///
/// Use wiremock to simulate a delay from Swapi.
///
/// ## Useful resources 📚
/// - [set_delay](https://docs.rs/wiremock/0.5.14/wiremock/struct.ResponseTemplate.html#method.set_delay)
/// - [timeout](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.timeout)
#[tokio::test]
async fn swapi_client_returns_timeout_error_if_timeout() {
    let luke = Person {
        name: "Luke Skywalker".to_string(),
        height: "172".to_string(),
    };
    // For this test to pass, you need to edit the `SwapiClient` to
    // take into account this timeout.
    let timeout = Duration::from_secs(2);

    // Start a `MockServer` and mock the GET request you do in the `SwapiClient`.
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/people/"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-type", "application/json")
                .set_body_json(json!({"count": 0, "results": []}))
                .set_delay(Duration::from_secs(6000)),
        )
        .mount(&mock_server)
        .await;

    let base_url = mock_server.uri();
    let swapi_client = SwapiClient::new(base_url, timeout).unwrap();
    let err: reqwest::Error = swapi_client.people_by_name(&luke.name).await.unwrap_err();
    assert!(err.is_timeout());
}
