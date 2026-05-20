mod report;
mod ui;

use xilem::{EventLoop, EventLoopBuilder, WindowOptions, Xilem};

use crate::ui::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    run(EventLoop::with_user_event())
}

fn run(event_loop: EventLoopBuilder) -> Result<(), anyhow::Error> {
    let app = Xilem::new_simple(AppState::default(), app_logic, WindowOptions::new("Glurep"));
    app.run_in(event_loop)?;

    Ok(())
}
