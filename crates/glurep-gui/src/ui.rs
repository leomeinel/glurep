pub(crate) mod prelude {
    pub(super) use super::palette::*;
    pub(super) use super::widgets::root_view;
    pub(crate) use super::{AppState, ConfigTab, app_logic};
}

mod palette;
mod widgets;

use std::{fs, path::PathBuf, sync::Arc, time::Duration};

use anyhow::Context as _;
use glurep_core::prelude::*;
use usvg::Tree;
use xilem::{WidgetView, core::fork, tokio::time, view::task};

use crate::ui::prelude::*;

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
#[derive(Debug)]
pub(crate) struct SvgPagination {
    /// Current page index.
    pub(crate) index: usize,
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
            svgs: Default::default(),
            tree,
        }
    }
}

/// Main state used while running the app.
#[derive(Default, Debug)]
pub(crate) struct AppState {
    pub(crate) tab: ConfigTab,
    pub(crate) input_path: Option<PathBuf>,
    pub(crate) output_path: Option<PathBuf>,
    pub(crate) patient_name: String,
    pub(crate) plot_config: PlotConfig,
    pub(crate) page_config: PageConfig,
    pub(crate) svg_pagination: SvgPagination,
}
impl AppState {
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

    /// Data needed for displaying svgs.
    ///
    /// [`SvgPagination::svgs`] and [`SvgPagination::tree`] can be updated from this.
    fn svg_display_data(
        &self,
        opt: Arc<usvg::Options<'_>>,
    ) -> Result<Option<(Vec<SvgData>, Tree)>, anyhow::Error> {
        let Some(input_path) = self.input_path.as_ref() else {
            return Ok(None);
        };
        let readings_map = readings_map(input_path)
            .context(format!("Failed to deserialize `{}`", input_path.display()))?;
        let svgs = plot_to_strings(&readings_map, &self.plot_config)
            .context(format!("Failed to plot `{}`", input_path.display()))?;

        let Some(svg) = svgs.get(self.svg_pagination.index) else {
            return Ok(None);
        };
        let svg_tree = Tree::from_str(svg.contents.as_str(), &opt)?;

        Ok(Some((svgs, svg_tree)))
    }
}

/// App logic responsible for running the app.
///
/// This also handles updates to the displayed `svg` and writing files.
pub(crate) fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    fork(
        root_view(state),
        (
            // FIXME: Find a way to only run this if state has changed without caching or having to set a trigger manually.
            //        This currently produces a visual bug where points are sometimes rendered in front of neighboring ones,
            //        other times behind because they are not necessarily rendered in the same order.
            task(
                |proxy, _| async move {
                    let mut interval = time::interval(Duration::from_millis(250));

                    let mut opt = usvg::Options::default();
                    opt.fontdb_mut().load_system_fonts();
                    let opt = Arc::new(opt);

                    loop {
                        interval.tick().await;
                        if proxy.message(opt.clone()).is_err() {
                            break;
                        };
                    }
                },
                |state: &mut AppState, opt| match state.svg_display_data(opt) {
                    Ok(Some((svgs, svg_tree))) => {
                        state.svg_pagination.svgs = Some(svgs);
                        state.svg_pagination.tree = Arc::new(svg_tree);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                    _ => (),
                },
            ),
            if let Err(e) = state.write_files() {
                eprintln!("Error: {}", e)
            },
        ),
    )
}
