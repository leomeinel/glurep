pub(crate) mod prelude {
    pub(super) use super::palette::*;
    pub(super) use super::widgets::root_view;
    pub(crate) use super::{AppState, ConfigTab, app_logic};
}

mod palette;
mod widgets;

use std::{path::PathBuf, sync::Arc, time::Duration};

use glurep_core::prelude::*;
use usvg::Tree;
use xilem::{WidgetView, core::fork, tokio::time, view::task};

use crate::{report::prelude::*, ui::prelude::*};

/// Enum representation of a tab containing configuration views.
#[repr(usize)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConfigTab {
    /// Tab for customizing [`PlotConfig`].
    #[default]
    Plot,
    /// Tab for customizing [`PageConfig`].
    Page,
}

/// Pagination data for `svg` view.
pub(crate) struct SvgPagination {
    /// Current page index.
    pub(crate) index: usize,
    /// Last page index.
    pub(crate) last_index: Option<usize>,
    /// [`SvgData`] cache.
    pub(crate) svgs: Option<Vec<SvgData>>,
    /// [`Tree`] that gets displayed in an `svg` view.
    pub(crate) tree: Arc<Tree>,
}
impl Default for SvgPagination {
    fn default() -> Self {
        let tree = Arc::new(Tree::from_str("<svg />", &usvg::Options::default()).unwrap());

        Self {
            index: Default::default(),
            last_index: Default::default(),
            svgs: Default::default(),
            tree,
        }
    }
}

/// Main state used while running the app.
#[derive(Default)]
pub(crate) struct AppState {
    pub(crate) tab: ConfigTab,
    pub(crate) input_path: Option<PathBuf>,
    pub(crate) output_path: Option<PathBuf>,
    pub(crate) patient_name: String,
    pub(crate) plot_config: PlotConfig,
    pub(crate) last_plot_config: Option<PlotConfig>,
    pub(crate) page_config: PageConfig,
    pub(crate) svg_pagination: SvgPagination,
}
impl AppState {
    /// Returns whether the `svg` should be redrawn.
    fn should_redraw_svg(&self) -> bool {
        self.last_plot_config
            .as_ref()
            .is_none_or(|p| &self.plot_config != p)
            || self
                .svg_pagination
                .last_index
                .is_none_or(|o| o != self.svg_pagination.index)
    }
    /// Write files to either `pdf` or a directory of `svgs` according to `output_path`.
    fn write_files(&mut self) -> Result<(), anyhow::Error> {
        if let Some(output_path) = self.output_path.as_ref()
            && let Some(svgs) = self.svg_pagination.svgs.as_ref()
        {
            if output_path.is_dir() {
                for svg in svgs {
                    let file_name = format!("{}.svg", svg.date);
                    let output_path = output_path.join(file_name);
                    fs::write(output_path, &svg.contents)?;
                }
            } else {
                let pdf_bytes =
                    svgs_to_pdf_bytes(svgs, &self.page_config, self.patient_name.as_str())
                        .context(format!("Failed to create pdf `{}`", output_path.display()))?;
                fs::write(&output_path, pdf_bytes)?;
            }
        }
        // NOTE: Reset `output_path` to only export once just after closing the `FileDialog`.
        self.output_path = None;

        Ok(())
    }
    /// Update [`svgs`](SvgPagination::svgs).
    fn update_svgs(&mut self) -> Result<(), anyhow::Error> {
        if let Some(input_path) = self.input_path.as_ref() {
            self.svg_pagination.svgs = Some(svg_data(&self.plot_config, input_path)?);
        }

        Ok(())
    }
    /// Update [`svg tree`](SvgPagination::tree).
    fn update_svg_tree(&mut self) -> Result<(), anyhow::Error> {
        if let Some(svgs) = self.svg_pagination.svgs.as_ref() {
            let mut options = usvg::Options::default();
            options.fontdb_mut().load_system_fonts();
            let Some(svg) = svgs.get(self.svg_pagination.index) else {
                return Err(PlotError::InsufficientSvgs.into());
            };
            let svg_contents = svg.contents.as_str();

            self.svg_pagination.tree = Arc::new(Tree::from_str(svg_contents, &options)?);

            self.last_plot_config = Some(self.plot_config.clone());
            self.svg_pagination.last_index = Some(self.svg_pagination.index);
        }

        Ok(())
    }
}

/// App logic responsible for running the app.
///
/// This also handles updates to the displayed `svg` and writing files.
pub(crate) fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    fork(
        root_view(state),
        (
            task(
                |proxy, _| async move {
                    let mut interval = time::interval(Duration::from_millis(200));
                    loop {
                        interval.tick().await;
                        if proxy.message(()).is_err() {
                            break;
                        };
                    }
                },
                // FIXME: Make this async
                |state: &mut AppState, _| {
                    if state.should_redraw_svg() {
                        if let Err(e) = state.update_svgs() {
                            eprintln!("Error: {}", e);
                        }
                        if let Err(e) = state.update_svg_tree() {
                            eprintln!("Error: {}", e);
                        }
                    }
                },
            ),
            // FIXME: Make this async
            if let Err(e) = state.write_files() {
                eprintln!("Error: {}", e)
            },
        ),
    )
}
