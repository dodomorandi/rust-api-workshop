use crate::swapi::{Person, SwapiClient};

const YODA_HEIGHT: u32 = 66;

pub struct YodaTaller {
    client: SwapiClient,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YodaTallerOutcome {
    pub person: String,
    pub taller: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum YodaTallerError {
    #[error("Unexpected error from reqwest: {0}")]
    UnexpectedError(#[from] reqwest::Error),

    #[error("Person not found")]
    PersonNotFound,

    #[error("Height not found or invalid")]
    HeightNotFound,
}

impl YodaTaller {
    pub fn new(swapi_client: SwapiClient) -> Self {
        Self {
            client: swapi_client,
        }
    }

    pub async fn is_taller_than(&self, name: &str) -> Result<YodaTallerOutcome, YodaTallerError> {
        // let yoda_height = match self.yoda_height.get() {
        //     Some(height) => height,
        //     None => {
        //         let mut people = self.client.people_by_name("Yoda").await?.into_iter();
        //         let yoda = match people.next() {
        //             Some(yoda) => yoda,
        //             None => panic!("Yoda must exist!!!"),
        //         };

        //         if people.next().is_some() {
        //             panic!("There can be only one Yoda!!!");
        //         }

        //         let yoda_height = yoda.height.parse().expect("height must be an integer");

        //         self.yoda_height.set(Some(yoda_height));
        //         yoda_height
        //     }
        // };

        let mut people = self.client.people_by_name(name).await?.into_iter();
        let Person { name, height } = people.next().ok_or(YodaTallerError::PersonNotFound)?;
        let height: u32 = height.parse().map_err(|_| YodaTallerError::HeightNotFound)?;

        Ok(YodaTallerOutcome {
            person: name,
            taller: YODA_HEIGHT > height,
        })
    }
}
