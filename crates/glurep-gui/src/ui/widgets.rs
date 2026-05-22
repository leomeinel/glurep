/*
 * Heavily inpired by:
 * - https://github.com/linebender/xilem/blob/main/xilem/examples/emoji_picker.rs
 * - https://github.com/linebender/xilem/blob/main/xilem/examples/slider_demo.rs
 * - https://github.com/linebender/xilem/blob/main/xilem/examples/stopwatch.rs
 * - https://github.com/linebender/xilem/blob/main/xilem/examples/widgets.rs
 */

use std::{ops::Range, path::PathBuf};

use glurep_core::prelude::{PageConfig, PlotConfig};
use rfd::FileDialog;
use xilem::{
    FontWeight, WidgetView,
    core::{lens, map_state},
    masonry::layout::{AsUnit as _, Dim},
    style::Style,
    view::{
        CrossAxisAlignment, FlexExt, FlexSpacer, MainAxisAlignment, ObjectFit, flex_col, flex_row,
        indexed_stack, label, slider, svg, text_button, text_input,
    },
};

use crate::ui::prelude::*;

/// Font size for headers.
const HEADER_FONT_SIZE: f32 = 20.;
/// [`FontWeight`] for headers.
const HEADER_FONT_WEIGHT: FontWeight = FontWeight::SEMI_BOLD;

/// Minimum delta between ranges defined by a [`slider`].
const MIN_SLIDER_RANGE_DELTA: f64 = 1.;

/// Root view containing any views of the app window.
pub(crate) fn root_view(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex_row((
        flex_col((
            flex_row((
                text_button("Plot Options", |state: &mut AppState| {
                    state.tab = ConfigTab::Plot
                })
                .disabled(state.tab == ConfigTab::Plot),
                text_button("Pdf Options", |state: &mut AppState| {
                    state.tab = ConfigTab::Page
                })
                .disabled(state.tab == ConfigTab::Page),
            ))
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .gap(10.px()),
            indexed_stack((
                flex_col((
                    //plot_config_view(state),
                    lens(
                        |plot_config: &mut PlotConfig| plot_config_view(plot_config),
                        |state: &mut AppState| &mut state.plot_config,
                    ),
                    lens(
                        |_| input_path_button(),
                        |state: &mut AppState| &mut state.input_path,
                    ),
                    map_state(
                        ouput_svg_button(&state.input_path),
                        |state: &mut AppState| &mut state.output_path,
                    ),
                ))
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .gap(10.px()),
                flex_col((
                    lens(
                        |page_config| page_config_view(page_config),
                        |state: &mut AppState| &mut state.page_config,
                    ),
                    lens(
                        |patient_name: &mut String| patient_name_text_input(patient_name),
                        |state: &mut AppState| &mut state.patient_name,
                    ),
                    lens(
                        |_| input_path_button(),
                        |state: &mut AppState| &mut state.input_path,
                    ),
                    map_state(
                        output_pdf_button(&state.input_path),
                        |state: &mut AppState| &mut state.output_path,
                    ),
                ))
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .gap(10.px()),
            ))
            .active(state.tab as usize),
        ))
        .gap(15.px()),
        FlexSpacer::Fixed(10.px()),
        flex_col((
            svg(state.svg_pagination.tree.clone())
                .fit(ObjectFit::Contain)
                .dims(Dim::Ratio(0.9))
                .padding(8.px()),
            map_state(
                pagination_view(
                    state.svg_pagination.index,
                    state.svg_pagination.svgs.as_ref().map_or(0, |s| s.len()),
                ),
                |state: &mut AppState| &mut state.svg_pagination.index,
            ),
        ))
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .gap(10.px())
        .dims(Dim::Stretch)
        .background(CONTENT_BACKGROUND)
        .corner_radius(8.px())
        .flex(1.),
    ))
    .gap(20.px())
    .padding(15.px())
    .background(ROOT_BACKGROUND)
}

/// Pagination view with a `size` of pages and an `index`.
///
/// This increase or decreases `index` by one from button interactions according to `size`.
fn pagination_view(index: usize, size: usize) -> impl WidgetView<usize> {
    flex_row((
        text_button("⬅️", |index: &mut usize| {
            *index = index.saturating_sub(1);
        })
        .disabled(index == 0),
        label(format!("{}/{}", index + 1, size)),
        text_button("➡️", |index: &mut usize| {
            *index += 1;
        })
        .disabled(index == size.saturating_sub(1)),
    ))
    .main_axis_alignment(MainAxisAlignment::Center)
    .cross_axis_alignment(CrossAxisAlignment::Center)
}

/// [`PageConfig`] view with sliders for setting all fields.
fn page_config_view(page_config: &PageConfig) -> impl WidgetView<PageConfig> + use<> {
    flex_col((
        label("Size")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        flex_col((
            ranged_slider(
                "X",
                page_config.size.0.0 as f64,
                0_f64..1000_f64,
                |page_config: &mut PageConfig, input| page_config.size.0.0 = input as f32,
            ),
            ranged_slider(
                "Y",
                page_config.size.1.0 as f64,
                0_f64..1000_f64,
                |page_config: &mut PageConfig, input| page_config.size.1.0 = input as f32,
            ),
        )),
        label("Margin")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        ranged_slider(
            "",
            page_config.margin.0 as f64,
            0_f64..50_f64,
            |page_config: &mut PageConfig, input| page_config.margin.0 = input as f32,
        ),
        label("Header Font Size")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        ranged_slider(
            "",
            page_config.header_font_size.0 as f64,
            0_f64..200_f64,
            |page_config: &mut PageConfig, input| page_config.header_font_size.0 = input as f32,
        ),
    ))
    .main_axis_alignment(MainAxisAlignment::Center)
    .cross_axis_alignment(CrossAxisAlignment::Center)
}

/// [`PlotConfig`] view with sliders for setting all fields.
fn plot_config_view(plot_config: &PlotConfig) -> impl WidgetView<PlotConfig> + use<> {
    flex_col((
        label("Y Axis Spec")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        flex_col((
            ranged_slider(
                "Min",
                plot_config.y_spec.start as f64,
                // NOTE: This ensures that start does not overflow end with a delta of `SLIDER_RANGE_DELTA`.
                //       Clamping additionally ensures that end does not overflow `range.start`.
                0_f64..(plot_config.y_spec.end as f64 - MIN_SLIDER_RANGE_DELTA).max(0.1),
                |plot_config: &mut PlotConfig, input| plot_config.y_spec.start = input as u32,
            ),
            ranged_slider(
                "Max",
                plot_config.y_spec.end as f64,
                // NOTE: This ensures that end does not overflow start with a delta of `SLIDER_RANGE_DELTA`.
                //       Clamping additionally ensures that end does not overflow `range.start`.
                (plot_config.y_spec.start as f64 + MIN_SLIDER_RANGE_DELTA).min(999.9)..1000_f64,
                |plot_config: &mut PlotConfig, input| plot_config.y_spec.end = input as u32,
            ),
        )),
        label("Number of Labels")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        flex_col((
            ranged_slider(
                "X",
                plot_config.num_labels.0 as f64,
                1_f64..24_f64,
                |plot_config: &mut PlotConfig, input| plot_config.num_labels.0 = input as usize,
            ),
            ranged_slider(
                "Y",
                plot_config.num_labels.1 as f64,
                1_f64..24_f64,
                |plot_config: &mut PlotConfig, input| plot_config.num_labels.1 = input as usize,
            ),
        )),
        label("Label Size")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        flex_col((
            ranged_slider(
                "X",
                plot_config.label_size.0 as f64,
                0_f64..100_f64,
                |plot_config: &mut PlotConfig, input| plot_config.label_size.0 = input as u32,
            ),
            ranged_slider(
                "Y",
                plot_config.label_size.1 as f64,
                0_f64..100_f64,
                |plot_config: &mut PlotConfig, input| plot_config.label_size.1 = input as u32,
            ),
        )),
        label("Glucose Target Range")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        flex_col((
            ranged_slider(
                "Low",
                plot_config.glucose_threshold.start as f64,
                // NOTE: This ensures that start does not overflow end with a delta of `SLIDER_RANGE_DELTA`.
                //       Clamping additionally ensures that end does not overflow `range.start`.
                0_f64..(plot_config.glucose_threshold.end as f64 - MIN_SLIDER_RANGE_DELTA).max(0.1),
                |plot_config: &mut PlotConfig, input| {
                    plot_config.glucose_threshold.start = input as u32
                },
            ),
            ranged_slider(
                "High",
                plot_config.glucose_threshold.end as f64,
                // NOTE: This ensures that end does not overflow start with a delta of `SLIDER_RANGE_DELTA`.
                //       Clamping additionally ensures that end does not overflow `range.start`.
                (plot_config.glucose_threshold.start as f64 + MIN_SLIDER_RANGE_DELTA).min(299.9)
                    ..300_f64,
                |plot_config: &mut PlotConfig, input| {
                    plot_config.glucose_threshold.end = input as u32
                },
            ),
        )),
        label("Point Radius")
            .weight(HEADER_FONT_WEIGHT)
            .text_size(HEADER_FONT_SIZE),
        ranged_slider(
            "",
            plot_config.point_radius as f64,
            0_f64..10_f64,
            |plot_config: &mut PlotConfig, input| plot_config.point_radius = input as u32,
        ),
    ))
    .main_axis_alignment(MainAxisAlignment::Center)
    .cross_axis_alignment(CrossAxisAlignment::Center)
}

/// View for setting [`AppState::patient_name`] via [`text_input`].
fn patient_name_text_input(patient_name: &mut String) -> impl WidgetView<String> + use<> {
    text_input(patient_name.clone(), |patient_name: &mut String, input| {
        *patient_name = input
    })
    .placeholder("Patient name")
}

/// View for opening a [`FileDialog`] to select an input [`PathBuf`] for csvs via a button.
fn input_path_button() -> impl WidgetView<Option<PathBuf>> + use<> {
    text_button("Select input file", |input_path: &mut Option<PathBuf>| {
        FileDialog::new()
            .add_filter("csv", &["csv"])
            .pick_file()
            .map(|p| *input_path = Some(p));
    })
}

/// View for opening a [`FileDialog`] to select an output [`PathBuf`] for pdf exports via a button.
fn output_pdf_button(input_path: &Option<PathBuf>) -> impl WidgetView<Option<PathBuf>> + use<> {
    text_button("Export PDF", |output_path: &mut Option<PathBuf>| {
        FileDialog::new()
            .set_file_name("output.pdf")
            .save_file()
            .map(|p| *output_path = Some(p));
    })
    .disabled(input_path.is_none())
}

/// View for opening a [`FileDialog`] to select an output [`PathBuf`] for svg exports to a directory via a button.
fn ouput_svg_button(input_path: &Option<PathBuf>) -> impl WidgetView<Option<PathBuf>> + use<> {
    text_button("Export SVGs", |output_path: &mut Option<PathBuf>| {
        FileDialog::new()
            .set_title("Select output directory")
            .pick_folder()
            .map(|p| *output_path = Some(p));
    })
    .disabled(input_path.is_none())
}

/// Customizable ranged slider with styling.
fn ranged_slider<S, F>(
    text: &'static str,
    value: f64,
    range: Range<f64>,
    on_change: F,
) -> impl WidgetView<S>
where
    S: 'static,
    F: Fn(&mut S, f64) + Send + Sync + 'static,
{
    flex_row((
        label(text).width(40.px()),
        slider(range.start, range.end, value, on_change).width(200.px()),
        label(format!("{:.0}", value)).width(60.px()),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .gap(10.px())
}
