use std::time::Duration;

use workshop::{swapi::SwapiClient, taller::YodaTaller};

use super::swapi_mock::SwapiMock;

pub struct TestApp {
    pub swapi_server: SwapiMock,
    pub swapi_client: SwapiClient,
    pub yoda_taller: YodaTaller,
}

pub const SWAPI_TIMEOUT: Duration = Duration::from_secs(2);

impl TestApp {
    pub async fn spawn() -> Self {
        let swapi_server = SwapiMock::start().await;
        let swapi_client = SwapiClient::new(swapi_server.uri(), SWAPI_TIMEOUT)
            .expect("swapi client should be able to connect to mock server");
        let yoda_taller = YodaTaller::new(swapi_client.clone());

        Self {
            swapi_server,
            swapi_client,
            yoda_taller,
        }
    }
}
