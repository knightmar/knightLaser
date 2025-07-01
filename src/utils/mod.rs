pub enum Error {
    FileNotFound(String),
    FileNotReadable(String),
    GeneralError(String),
}
