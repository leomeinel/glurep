pub(crate) mod prelude {
    pub(crate) use super::{svg_data, write_pdf, write_svgs};
}
use std::{fs, path::PathBuf};

use anyhow::Context as _;
use glurep_core::prelude::*;

/// Write pdf from `svgs` to a file.
pub(crate) fn write_pdf(
    page_config: &PageConfig,
    output_path: &PathBuf,
    svgs: &[SvgData],
    patient_name: &str,
) -> Result<(), anyhow::Error> {
    let total_margin = page_config.margin * 2.;
    if total_margin > page_config.size.0 || total_margin > page_config.size.1 {
        return Err(PdfError::PageConfigMarginExceedsSize.into());
    }

    let pdf_bytes = svgs_to_pdf_bytes(svgs, &page_config, patient_name)
        .context(format!("Failed to create pdf `{}`", output_path.display()))?;
    fs::write(&output_path, pdf_bytes)?;

    Ok(())
}

/// Write all `svgs` to multiple files.
///
/// The file name will contain the date.
pub(crate) fn write_svgs(output_path: &PathBuf, svgs: &[SvgData]) -> Result<(), anyhow::Error> {
    for svg in svgs {
        let file_name = format!("{}.svg", svg.date);
        let output_path = output_path.join(file_name);
        fs::write(output_path, &svg.contents)?;
    }

    Ok(())
}

/// Generate [`Vec<SvgData>`] from deserialized `csv` at `input_pat`.
pub(crate) fn svg_data(
    plot_config: &PlotConfig,
    input_path: &PathBuf,
) -> Result<Vec<SvgData>, anyhow::Error> {
    let readings_map = readings_map(input_path)
        .context(format!("Failed to deserialize `{}`", input_path.display()))?;

    plot_to_strings(&readings_map, plot_config)
        .context(format!("Failed to plot `{}`", input_path.display()))
}
