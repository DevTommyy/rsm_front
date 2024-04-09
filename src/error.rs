use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
pub enum Error {
    // -- Config errors
    FailedToReadConfig,
    InvalidConfig,
    FailedToUpdateConf,

    // -- Server errors
    FailedToConnectToServer,
    FailedtoReadServerResponse,
    InvalidServerResponse,

    // -- Other errors
    FirstRunFailed,
    NoAuth,
    RsmFailed,
    LoginFail,
    FailedToUpdateKey,
}
