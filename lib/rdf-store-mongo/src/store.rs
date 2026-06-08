// This is free and unencumbered software released into the public domain.

use crate::MongoError;
use derive_more::Debug;
use futures::executor::block_on;
use mongodb::Client;

/// The default localhost connection URL for MongoDB.
///
/// See: <https://www.mongodb.com/resources/products/compatibilities/docker>
pub const DEFAULT_URL: &str = "mongodb://localhost:27017";

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a MongoDB database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_mongo::MongoStore;
/// let mut store = MongoStore::open("mongodb://localhost:27017").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct MongoStore {
    pub client: Client,
}

impl MongoStore {
    pub async fn open(url: impl AsRef<str>) -> Result<Self, MongoError> {
        let client = Client::with_uri_str(url).await?;
        Ok(Self { client })
    }
}

impl Default for MongoStore {
    fn default() -> Self {
        block_on(Self::open(DEFAULT_URL)).expect("should connect to mongodb://localhost:27017")
    }
}
