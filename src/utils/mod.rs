#[derive(Clone)]
#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
    FileNotReadable(String),
    GeneralError(String),
    MotorError(String),
}

pub fn get_error_string(error: &Error) -> String {
    match error {
        Error::FileNotFound(msg)
        | Error::FileNotReadable(msg)
        | Error::GeneralError(msg)
        | Error::MotorError(msg) => msg.clone(),
    }
}