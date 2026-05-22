use xilem::Color;

/// Background color for root view.
pub(super) const ROOT_BACKGROUND: Color = SLATE_700;
/// Background color for content containing `svg` view.
pub(super) const CONTENT_BACKGROUND: Color = SLATE_400;

/// Tailwind `slate-400` - rgb(148, 163, 184)
const SLATE_400: Color = Color::from_rgb8(148, 163, 184);
/// Tailwind `slate-700` - rgb(51, 65, 85)
const SLATE_700: Color = Color::from_rgb8(51, 65, 85);
