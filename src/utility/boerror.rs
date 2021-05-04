use failure::Fail;

/// An enum of errors this crate may produce. These are compatible with
/// `failure` errors.
#[derive(Debug, Fail)]
pub enum BOError {
    #[fail(display = "manifest not available")]
    ManifestNotAvailable,
}
