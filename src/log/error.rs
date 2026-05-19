use std::path::PathBuf;

/// Error if an invalid time is being plotted from csv.
pub(crate) const PLOT_ERR_INVALID_TIME: &str = "Invalid time encountered while plotting csv.";

/// [`Errors`](thiserror::Error) for invalid usage for specified flags.
#[derive(thiserror::Error, Debug)]
pub(crate) enum FlagError {
    #[error("File `{0}` already exists. Use `--force` to overwrite.")]
    IoErrorAlreadyExists(PathBuf),
    #[error("`{0}` is not a directory. For `--svg` you need to specify a directory.")]
    IoErrorNotADirectory(PathBuf),
    #[error("`{0}` is a directory. If not using `--svg` you need to specify a file.")]
    IoErrorIsADirectory(PathBuf),
}

/// [`Errors`](thiserror::Error) encountered in [`crate::pdf`].
#[derive(thiserror::Error, Debug)]
pub(crate) enum PdfError {
    #[error("Failed to determine min/max date of glucose readings.")]
    MinMaxErrorReadings,
}
