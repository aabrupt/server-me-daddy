pub enum SystemError {
    IoError(std::io::Error),
    AlreadySpawned,
}
