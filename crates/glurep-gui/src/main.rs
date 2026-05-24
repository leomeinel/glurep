mod ui;

use xilem::{EventLoop, WindowOptions, Xilem};

use crate::ui::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let app = Xilem::new_simple(AppState::default(), app_logic, WindowOptions::new("Glurep"));
    app.run_in(EventLoop::with_user_event())?;

    Ok(())
}
