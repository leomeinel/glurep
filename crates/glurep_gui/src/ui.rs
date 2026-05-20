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
    style::Style,
    view::{flex_col, flex_row},
};

use crate::ui::prelude::*;

#[derive(Default)]
pub(crate) struct AppState {
    input_file: PathBuf,
    output_file: PathBuf,
    patient_name: String,
    plot_config: PlotConfig,
    page_config: PageConfig,
}

pub(crate) fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
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
    .padding(50.)
}
