use anyhow::Ok;
use root_rs::{all_modules::*, anyhow::Result};

fn main() -> Result<()> {
    let mut app = TApplication::new("app");

    let mut canvas = TCanvas::new_with_size("canvas", "simple 1D histogram", 400, 300);

    canvas.terminate_app_on_close()?;

    app.run_return();
    println!("Application terminated");
    Ok(())
}
