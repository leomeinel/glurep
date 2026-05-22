use anyhow::anyhow;
#[cfg(feature = "cli")]
use clap::ArgMatches;
use itertools::Itertools as _;
use jiff::civil::Date;
use printpdf::{
    BuiltinFont, Color, Mm, Op, PdfDocument, PdfFontHandle, PdfPage, PdfSaveOptions, Point, Pt, Px,
    Rgb, Svg, TextItem, XObjectTransform,
};

use crate::{log::prelude::*, plot::prelude::*};

/// Scale factor from [`Px::into_pt`](printpdf::units::Px::into_pt).
///
/// This is necessary to have the pixels of an svg match a millimeter if used as [`XObjectTransform::dpi`].
const PX_INTO_PT_DPI: f32 = 25.4;

/// Config for plotting an svg to a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageConfig {
    /// Page size `(x, y)`.
    pub size: (Mm, Mm),
    /// Page margin.
    pub margin: Mm,
    /// Header font size.
    pub header_font_size: Pt,
}
#[cfg(feature = "cli")]
impl From<&ArgMatches> for PageConfig {
    fn from(matches: &ArgMatches) -> Self {
        let mut config = PageConfig::default();
        matches
            .get_one::<u32>("size_x")
            .map(|&x| config.size.0 = Mm(x as f32));
        matches
            .get_one::<u32>("size_y")
            .map(|&y| config.size.1 = Mm(y as f32));
        matches
            .get_one::<u32>("margin")
            .map(|&m| config.margin = Mm(m as f32));
        matches
            .get_one::<f32>("header_font_size")
            .map(|&s| config.header_font_size = Pt(s));

        config
    }
}
impl Default for PageConfig {
    fn default() -> Self {
        Self {
            // NOTE: This is equivalent to ISO A4 size.
            size: (Mm(210.), Mm(297.)),
            margin: Mm(10.),
            header_font_size: Pt(20.),
        }
    }
}

/// [`PdfDocument`] from `svgs` as bytes.
pub fn svgs_to_pdf_bytes(
    svgs: &[SvgData],
    config: &PageConfig,
    patient_name: &str,
) -> Result<Vec<u8>, anyhow::Error> {
    let page_width = config.size.0;
    let page_height = config.size.1;

    let svg_transform = svg_transform(&config);
    let doc_name = doc_name(&svgs, patient_name)?;
    let mut doc = PdfDocument::new(doc_name.as_str());

    let mut pages = Vec::new();
    for svg_data in svgs {
        let svg =
            Svg::parse(svg_data.contents.as_str(), &mut Vec::new()).map_err(|e| anyhow!(e))?;
        let id = doc.add_xobject(&svg);
        let mut ops = Vec::new();
        ops.extend_from_slice(&header(&config, &svg_data.date, patient_name));
        ops.push(Op::UseXobject {
            id,
            transform: svg_transform,
        });

        pages.push(PdfPage::new(page_width, page_height, ops));
    }
    if pages.is_empty() {
        return Err(PdfError::EmptyPages.into());
    }

    let bytes = doc
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut Vec::new());
    if bytes.is_empty() {
        return Err(PdfError::EmptyBytes.into());
    }

    Ok(bytes)
}

/// Header containing `patient_name` and `date`.
fn header(config: &PageConfig, date: &Date, patient_name: &str) -> [Op; 9] {
    let page_height = config.size.1;
    let margin = config.margin;
    // FIXME: Visually the margin resulting from pos_y is not exactly the same as other margins if `margin` is less than `header_font_size / 2.`.
    // NOTE: The cursor seems to draw from the center, therefore we need to subtract half of the font size.
    let pos_y = page_height - margin - Mm::from(config.header_font_size / 2.);
    let text = format!("{}: {}", patient_name, date);

    [
        Op::SaveGraphicsState,
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(margin, pos_y),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::HelveticaBold),
            size: config.header_font_size,
        },
        Op::SetLineHeight {
            lh: config.header_font_size,
        },
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: 0.,
                g: 0.,
                b: 0.,
                icc_profile: None,
            }),
        },
        Op::ShowText {
            items: vec![TextItem::Text(text)],
        },
        Op::EndTextSection,
        Op::RestoreGraphicsState,
    ]
}

/// [`XObjectTransform`] to position svg in the middle of the page and fit [`PageConfig::margin`].
fn svg_transform(config: &PageConfig) -> XObjectTransform {
    let page_width = config.size.0;
    let page_height = config.size.1;
    let margin = config.margin;

    let content_width = page_width - margin * 2.;
    let content_height = page_height - margin * 2.;
    let svg_scale = (content_width.0 / SVG_SIZE.0 as f32).min(content_height.0 / SVG_SIZE.1 as f32);

    let width = Px((SVG_SIZE.0 as f32 * svg_scale) as usize).into_pt(PX_INTO_PT_DPI);
    let height = Px((SVG_SIZE.1 as f32 * svg_scale) as usize).into_pt(PX_INTO_PT_DPI);
    let translation = (
        (page_width.into_pt() - width) / 2.,
        (page_height.into_pt() - height) / 2.,
    );

    XObjectTransform {
        translate_x: Some(translation.0),
        translate_y: Some(translation.1),
        scale_x: Some(svg_scale),
        scale_y: Some(svg_scale),
        dpi: Some(PX_INTO_PT_DPI),
        ..Default::default()
    }
}

/// Name for a [`PdfDocument`] from `svgs` for `patient_name`.
fn doc_name(svgs: &[SvgData], patient_name: &str) -> Result<String, anyhow::Error> {
    let Some((min_date, max_date)) = svgs.iter().minmax().into_option() else {
        return Err(PdfError::MinMaxErrorReadings.into());
    };

    Ok(format!(
        "{}: {} - {}",
        patient_name, min_date.date, max_date.date
    ))
}
