use std::fmt::Debug;

pub enum Error {
    ReqwestError(reqwest::Error)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqwestError(e) => write!(f, "Reqwest Error: {:?}", e)
        }
    }
}