
#[allow(unused)]
use anyhow::{Result, anyhow};
use root_rs::core::{TApplication, TCanvas};
use root_rs::all_modules::*;

fn main() -> Result<()> {
    println!("Hello, world!");

    //const S: &str = concat!(env!("OUT_DIR"), "/bindings.rs");
    //println!("S: {}", S);

    let mut app = TApplication::new("app");

    let mut canvas = TCanvas::new_with_size("a", "b", 400, 300);
    canvas.terminate_app_on_close()?;

    //let app = root_rs::core::TApplication::gApplication().ok_or(anyhow!("No gApplication"))?;

    let mut histo = TH1F::new_range("ciao", "histo", 10, 0.0..=10.0);
    histo.as_TObject_mut().draw();
    let mut histo = TH1F::new_range("ciao 2", "histo 2", 10, 0.0..=20.0);
    histo.as_TH1_mut().add_bin_content(1);
    histo.add_bin_content(1);
    histo.draw();

    app.run_return();

    Ok(())
}