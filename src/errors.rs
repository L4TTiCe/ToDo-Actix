use mongodb::bson;
use mongodb::error::Error as MongoError;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ParseObjectIDFailed(#[from] bson::oid::Error),

    #[error("ENV VARIABLE for `{0}` is not set")]
    EnvironmentVariableNotSet(String),

    #[error("{0}")]
    Mongo(#[from] MongoError),

    #[error("{0}")]
    Bson(#[from] bson::de::Error),

    #[error("{0}")]
    ServerStartFailed(#[from] std::io::Error),

    #[error("Resource at `{0}` not found")]
    NotFound(String),
}
