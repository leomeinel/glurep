pub(crate) mod prelude {
    pub(crate) use super::{PageConfig, doc_name, svgs_to_pdf_bytes};
}

use std::error::Error;

use itertools::Itertools as _;
use printpdf::{Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Svg, XObjectTransform};

use crate::{deserialize::GlucoseReadingsMap, log::prelude::*, plot::prelude::*};

/// Scale factor from [`Px::into_pt`](printpdf::units::Px::into_pt).
///
/// This is necessary to have the pixels match a millimeter if used as [`XObjectTransform::dpi`].
const PX_INTO_PT_SCALE: f32 = 25.4;

/// Config for plotting an svg to a page..
#[derive(Clone, Debug)]
pub(crate) struct PageConfig {
    /// Size of the page `(x, y)` in pixels.
    pub(crate) size: (Mm, Mm),
}
impl From<PlotConfig> for PageConfig {
    fn from(config: PlotConfig) -> Self {
        Self {
            size: (Mm(config.size.0 as f32), Mm(config.size.1 as f32)),
        }
    }
}

/// [`PdfDocument`] from `svgs` with `doc_name` as bytes.
pub(crate) fn svgs_to_pdf_bytes(
    svgs: Vec<String>,
    config: PageConfig,
    doc_name: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut doc = PdfDocument::new(doc_name);
    let mut pages = Vec::new();
    for svg in svgs {
        let svg = Svg::parse(svg.as_str(), &mut Vec::new())?;
        let id = doc.add_xobject(&svg);
        let ops = vec![Op::UseXobject {
            id,
            transform: XObjectTransform {
                dpi: Some(PX_INTO_PT_SCALE),
                ..Default::default()
            },
        }];

        pages.push(PdfPage::new(config.size.0, config.size.1, ops));
    }

    let bytes = doc
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new());
    Ok(bytes)
}

/// Name for a [`PdfDocument`] from `readings` for `patient_name`.
pub(crate) fn doc_name(readings: &GlucoseReadingsMap, patient_name: &str) -> String {
    let (min_date, max_date) = readings
        .0
        .keys()
        .minmax()
        .into_option()
        .expect(ERR_INVALID_READINGS);

    format!("{}: {} - {}", patient_name, min_date, max_date)
}
