use anyhow::{Result, anyhow};
use root_rs::core::{TApplication, TCanvas, TRootCanvas};

fn main() -> Result<()> {
    println!("Hello, world!");

    const S: &str = concat!(env!("OUT_DIR"), "/bindings.rs");
    println!("S: {}", S);

    let app = TApplication::new("app");

    let mut canvas = TCanvas::new_name_title_width_height("a", "b", 400, 300);
    canvas.terminate_app_on_close()?;

    let app = root_rs::core::TApplication::gApplication().ok_or(anyhow!("No gApplication"))?;

    app.run_return();

    Ok(())
}