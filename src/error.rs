use thiserror::Error;

#[derive(Error, Debug)]
pub enum UspError {
    #[error("to_id must be set before building the record")]
    ToIdNotSet,
    #[error("Record type must be set before building the record")]
    RecordTypeNotSet,
    #[error(transparent)]
    EncodeError(#[from] prost::DecodeError),
}
