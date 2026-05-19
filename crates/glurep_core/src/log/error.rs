use std::path::PathBuf;

/// Error if an invalid time is being plotted from csv.
pub(crate) const PLOT_ERR_INVALID_TIME: &str = "Invalid time encountered while plotting csv.";

/// [`Errors`](thiserror::Error) for invalid usage for specified flags.
#[derive(thiserror::Error, Debug)]
pub enum FlagError {
    #[error("File `{0}` already exists. Use `--force` to overwrite.")]
    IoErrorAlreadyExists(PathBuf),
    #[error("`{0}` is not a directory. For `--svg` you need to specify a directory.")]
    IoErrorNotADirectory(PathBuf),
    #[error("`{0}` is a directory. If not using `--svg` you need to specify a file.")]
    IoErrorIsADirectory(PathBuf),
}

/// [`Errors`](thiserror::Error) encountered in [`crate::deserialize`].
#[derive(thiserror::Error, Debug)]
pub enum DeserializeError {
    #[error("Failed to read any valid entries from csv.")]
    EmptyGlucoseReadingsMap,
}

/// [`Errors`](thiserror::Error) encountered in [`crate::plot`].
#[derive(thiserror::Error, Debug)]
pub enum PlotError {
    #[error("Failed to construct svgs.")]
    EmptySvgs,
}

/// [`Errors`](thiserror::Error) encountered in [`crate::pdf`].
#[derive(thiserror::Error, Debug)]
pub enum PdfError {
    #[error("Failed to serialize pdf document to bytes.")]
    EmptyBytes,
    #[error("Failed to construct pages for pdf document.")]
    EmptyPages,
    #[error("Failed to determine min/max date of glucose readings.")]
    MinMaxErrorReadings,
    #[error("Total margin can not exceed width or height.")]
    PageConfigMarginExceedsSize,
}
