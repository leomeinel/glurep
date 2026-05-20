pub(crate) mod prelude {
    pub(super) use super::widgets::{
        file_dialog_spacer_panel, input_file_panel, output_file_panel, page_config_panel,
        plot_config_panel, text_input_panel,
    };
    pub(crate) use super::{AppState, app_logic};
}

mod widgets;

use std::path::PathBuf;

use glurep_core::prelude::*;
use xilem::{
    WidgetView,
    core::fork,
    style::Style,
    view::{flex_col, flex_row},
};

use crate::{report::prelude::*, ui::prelude::*};

#[derive(Default)]
pub(crate) struct AppState {
    pub(crate) input_path: Option<PathBuf>,
    pub(crate) output_path: Option<PathBuf>,
    pub(crate) patient_name: String,
    pub(crate) plot_config: PlotConfig,
    pub(crate) page_config: PageConfig,
}
impl AppState {
    fn export_files(&mut self) {
        if let Some(input_path) = self.input_path.as_ref()
            && let Some(output_path) = self.output_path.as_ref()
        {
            if output_path.is_dir() {
                // FIXME: Do not use unwrap here.
                export_svgs(self, input_path, output_path).unwrap();
            } else {
                // FIXME: Do not use unwrap here.
                export_pdf(self, input_path, output_path).unwrap();
            }
        } else if self.output_path.is_some() {
            todo!("Show warning");
        }

        // NOTE: Reset `output_path` to only export once just after closing the `FileDialog`.
        self.output_path = None;
    }
}

pub(crate) fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    fork(
        flex_col((
            plot_config_panel(state),
            page_config_panel(state),
            text_input_panel(state),
            flex_row((
                input_file_panel(),
                file_dialog_spacer_panel(),
                output_file_panel(),
            )),
        ))
        .padding(50.),
        state.export_files(),
    )
}
