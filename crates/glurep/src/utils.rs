use std::{env, path::PathBuf};

pub(crate) mod prelude {
    pub(crate) use super::file_dialog_path;
}

pub(crate) fn file_dialog_path() -> PathBuf {
    env::home_dir().unwrap_or(env::temp_dir())
}
