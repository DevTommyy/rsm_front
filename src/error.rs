use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
    FailedToReadConfig,
    InvalidConfig,
    FailedToUpdateConf,
    FirstRunFailed,
    NoAuth,
    FailedToConnectToServer,
    FailedtoReadServerResponse,
    InvalidServerResponse,
}
