mod deserialize;
mod log;
mod pdf;
mod plot;
mod utils;

pub mod prelude {
    pub use crate::deserialize::{GlucoseReading, GlucoseReadingsMap, readings_map};
    pub use crate::log::error::{DeserializeError, FlagError, PdfError, PlotError};
    pub use crate::pdf::{PageConfig, svgs_to_pdf_bytes};
    pub use crate::plot::{PlotConfig, SvgData, plot_to_strings};
}
