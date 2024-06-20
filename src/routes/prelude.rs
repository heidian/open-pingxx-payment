use crate::utils::DBError;
use crate::core::{OrderError, ChargeError};

impl From<DBError> for OrderError {
    fn from(e: DBError) -> Self {
        match e {
            DBError::SQLFailed(msg) => OrderError::Unexpected(msg),
            DBError::DoesNotExist(msg) => OrderError::BadRequest(msg),
        }
    }
}

impl From<DBError> for ChargeError {
    fn from(e: DBError) -> Self {
        match e {
            DBError::SQLFailed(msg) => ChargeError::InternalError(msg),
            DBError::DoesNotExist(msg) => ChargeError::MalformedRequest(msg),
        }
    }
}
