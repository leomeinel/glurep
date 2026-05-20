pub(crate) mod prelude {
    pub(crate) use super::{export_pdf, export_svgs};
}
use std::{fs, path::PathBuf};

use anyhow::Context as _;
use glurep_core::prelude::*;

use crate::ui::prelude::*;

pub(crate) fn export_pdf(
    state: &AppState,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let pdf_bytes = pdf_bytes(state, input_path, output_path)?;
    fs::write(&output_path, pdf_bytes)?;

    Ok(())
}

pub(crate) fn export_svgs(
    state: &AppState,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let svgs = svg_strings(&state.plot_config, input_path)?;
    for svg in svgs {
        let file_name = format!("{}.svg", svg.date);
        let output_path = output_path.join(file_name);
        fs::write(output_path, svg.contents)?;
    }

    Ok(())
}

fn pdf_bytes(
    state: &AppState,
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<Vec<u8>, anyhow::Error> {
    let page_config = &state.page_config;
    let total_margin = page_config.margin * 2.;
    if total_margin > page_config.size.0 || total_margin > page_config.size.1 {
        return Err(PdfError::PageConfigMarginExceedsSize.into());
    }

    let svgs = svg_strings(&state.plot_config, input_path)?;
    svgs_to_pdf_bytes(svgs, &page_config, &state.patient_name)
        .context(format!("Failed to create pdf `{}`", output_path.display()))
}

fn svg_strings(
    plot_config: &PlotConfig,
    input_path: &PathBuf,
) -> Result<Vec<SvgData>, anyhow::Error> {
    let readings_map = readings_map(input_path)
        .context(format!("Failed to deserialize `{}`", input_path.display()))?;

    plot_to_strings(&readings_map, plot_config)
        .context(format!("Failed to plot `{}`", input_path.display()))
}
