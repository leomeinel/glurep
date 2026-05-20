use rfd::FileDialog;
use xilem::{
    FontWeight, WidgetView,
    masonry::properties::types::Length,
    view::{
        CrossAxisAlignment, FlexSpacer, flex_col, flex_row, label, slider, text_button, text_input,
    },
};

use crate::ui::prelude::*;

pub(super) const CONTAINER_GAP: Length = Length::const_px(12.0);
pub(super) const ELEMENT_GAP: Length = Length::const_px(4.0);

pub(super) fn page_config_panel(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_col((
        label("Size"),
        flex_row((
            label(format!("X: {}", state.page_config.size.0.0)),
            slider(
                0_f64,
                1000_f64,
                state.page_config.size.0.0 as f64,
                |state: &mut AppState, input| state.page_config.size.0.0 = input as f32,
            )
            .step(1.),
            label(format!("Y: {}", state.page_config.size.1.0)),
            slider(
                0_f64,
                1000_f64,
                state.page_config.size.1.0 as f64,
                |state: &mut AppState, input| state.page_config.size.1.0 = input as f32,
            )
            .step(1.),
        )),
        label("Margin"),
        flex_row((
            label(format!("{}", state.page_config.margin.0)),
            slider(
                0_f64,
                50_f64,
                state.page_config.margin.0 as f64,
                |state: &mut AppState, input| state.page_config.margin.0 = input as f32,
            )
            .step(1.),
        )),
        label("Header Font Size"),
        flex_row((
            label(format!("{}", state.page_config.header_font_size.0)),
            slider(
                0_f64,
                200_f64,
                state.page_config.header_font_size.0 as f64,
                |state: &mut AppState, input| state.page_config.header_font_size.0 = input as f32,
            )
            .step(1.),
        )),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}

pub(super) fn plot_config_panel(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_col((
        label("Y Axis Spec"),
        flex_row((
            label(format!("Min: {}", state.plot_config.y_spec.start)),
            slider(
                0_f64,
                1000_f64,
                state.plot_config.y_spec.start as f64,
                |state: &mut AppState, input| state.plot_config.y_spec.start = input as u32,
            )
            .step(1.),
            label(format!("Max: {}", state.plot_config.y_spec.end)),
            slider(
                0_f64,
                1000_f64,
                state.plot_config.y_spec.end as f64,
                |state: &mut AppState, input| state.plot_config.y_spec.end = input as u32,
            )
            .step(1.),
        )),
        label("Number of Labels"),
        flex_row((
            label(format!("X: {}", state.plot_config.num_labels.0)),
            slider(
                0_f64,
                50_f64,
                state.plot_config.num_labels.0 as f64,
                |state: &mut AppState, input| state.plot_config.num_labels.0 = input as usize,
            )
            .step(1.),
            label(format!("Y: {}", state.plot_config.num_labels.1)),
            slider(
                0_f64,
                50_f64,
                state.plot_config.num_labels.1 as f64,
                |state: &mut AppState, input| state.plot_config.num_labels.1 = input as usize,
            )
            .step(1.),
        )),
        label("Label Size"),
        flex_row((
            label(format!("X: {}", state.plot_config.label_size.0)),
            slider(
                0_f64,
                200_f64,
                state.plot_config.label_size.0 as f64,
                |state: &mut AppState, input| state.plot_config.label_size.0 = input as u32,
            )
            .step(1.),
            label(format!("Y: {}", state.plot_config.label_size.1)),
            slider(
                0_f64,
                200_f64,
                state.plot_config.label_size.1 as f64,
                |state: &mut AppState, input| state.plot_config.label_size.1 = input as u32,
            )
            .step(1.),
        )),
        label("Glucose Target Range"),
        flex_row((
            label(format!(
                "Low: {}",
                state.plot_config.glucose_threshold.start
            )),
            slider(
                0_f64,
                300_f64,
                state.plot_config.glucose_threshold.start as f64,
                |state: &mut AppState, input| {
                    state.plot_config.glucose_threshold.start = input as u32
                },
            )
            .step(1.),
            label(format!("High: {}", state.plot_config.glucose_threshold.end)),
            slider(
                0_f64,
                300_f64,
                state.plot_config.glucose_threshold.end as f64,
                |state: &mut AppState, input| {
                    state.plot_config.glucose_threshold.end = input as u32
                },
            )
            .step(1.),
        )),
        label("Point Radius"),
        flex_row((
            label(format!("{}", state.plot_config.point_radius)),
            slider(
                0_f64,
                20_f64,
                state.plot_config.point_radius as f64,
                |state: &mut AppState, input| state.plot_config.point_radius = input as u32,
            )
            .step(1.),
        )),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}

pub(super) fn text_input_panel(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_col((
        text_input(state.patient_name.clone(), |state: &mut AppState, input| {
            state.patient_name = input
        })
        .placeholder("Patient name"),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}

// FIXME: Think about using `AsyncFileDialog` instead.
pub(super) fn input_file_panel() -> impl WidgetView<AppState> + use<> {
    flex_col((text_button("Select input file", |state: &mut AppState| {
        FileDialog::new()
            .add_filter("csv", &["csv"])
            .pick_file()
            .map(|p| state.input_path = Some(p));
    }),))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}

// FIXME: Think about using `AsyncFileDialog` instead.
pub(super) fn output_file_panel() -> impl WidgetView<AppState> + use<> {
    flex_col((
        text_button("Export PDF to…", |state: &mut AppState| {
            FileDialog::new()
                .set_file_name("output.pdf")
                .save_file()
                .map(|p| state.output_path = Some(p));
        }),
        FlexSpacer::Fixed(ELEMENT_GAP),
        text_button("Export SVGs to…", |state: &mut AppState| {
            FileDialog::new()
                .set_title("Select output directory")
                .pick_folder()
                .map(|p| state.output_path = Some(p));
        }),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}

pub(super) fn file_dialog_spacer_panel() -> impl WidgetView<AppState> + use<> {
    flex_col((
        FlexSpacer::Fixed(CONTAINER_GAP),
        label("->").weight(FontWeight::BOLD).text_size(20.),
        FlexSpacer::Fixed(CONTAINER_GAP),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .gap(CONTAINER_GAP)
}
