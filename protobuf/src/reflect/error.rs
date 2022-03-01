#[derive(Debug, thiserror::Error)]
pub(crate) enum ReflectError {
    #[error("Message `{}` not found in files: {}", .0, .1)]
    MessageNotFoundInFiles(String, String),
}