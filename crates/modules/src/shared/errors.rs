use std::error::Error;

type BoxError = Box<dyn Error + Send + Sync + 'static>;

#[derive(thiserror::Error, Debug)]
#[error("unexpected error")]
pub struct UnexpectedError {
    #[source]
    source: BoxError,
}

impl UnexpectedError {
    pub fn new<E>(source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            source: Box::new(source),
        }
    }
}
