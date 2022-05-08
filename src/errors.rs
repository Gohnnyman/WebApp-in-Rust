use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Невірне значення: {0:?}")]
    InvalidValue(Vec<String>),

    #[error("Невірний зовнішній ключ: {0:?}")]
    InvalidForeignKey(String),

    #[error("Невірна дата")]
    InvalidDate,

    #[error("Ці поля мають бути заповнені: {0:?}")]
    NullValues(Vec<String>),
}
