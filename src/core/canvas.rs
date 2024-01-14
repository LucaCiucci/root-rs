use anyhow::{Result, anyhow};

use super::*;
use std::{ffi::*, ops::Deref};

root_object! {
    TCanvas
}

impl TCanvas {
    pub fn new_build(build: bool) -> Self {
        let ptr = unsafe {
            ffi::root_rs_TCanvas__new_build(build)
        };
        TCanvas::from_ptr(ptr).unwrap()
    }

    pub fn new_name_title_form(
        name: &str,
        title: &str,
        form: TCanvasForm,
    ) -> Self {
        let name = CString::new(name).unwrap();
        let name: *const c_char = name.as_ptr() as *const _;

        let title = CString::new(title).unwrap();
        let title: *const c_char = title.as_ptr() as *const _;

        let ptr = unsafe {
            ffi::root_rs_TCanvas__new_name_title_form(
                name,
                title,
                form.number(),
            )
        };
        TCanvas::from_ptr(ptr).unwrap()
    }

    pub fn new_name_title_width_height(
        name: &str,
        title: &str,
        width: i32,
        height: i32,
    ) -> Self {
        let name = CString::new(name).unwrap();
        let name: *const c_char = name.as_ptr() as *const _;

        let title = CString::new(title).unwrap();
        let title: *const c_char = title.as_ptr() as *const _;

        let ptr = unsafe {
            ffi::root_rs_TCanvas__new_name_title_width_height(
                name,
                title,
                width,
                height,
            )
        };
        TCanvas::from_ptr(ptr).unwrap()
    }

    pub fn get_canvas_impl<'s>(&'s mut self) -> Option<RootRef<'s, TCanvasImp>> {
        unsafe {
            RootRef::new(ffi::root_rs_TCanvas__GetCanvasImp(self.0))
        }
    }

    /// Connects the "CloseWindow()" signal to the "Terminate()" slot of the application.
    ///
    /// This can be used as a shorthand for:
    /// ```ignore
    /// canvas
    ///     .get_canvas_impl().unwrap()
    ///     .dyn_into_ref::<TRootCanvas>().unwrap()
    ///     .as_TQObject()
    ///     .connect("CloseWindow()", "TApplication", &app, "Terminate()")
    ///     .unwrap();
    /// ```
    pub fn terminate_app_on_close(&mut self) -> Result<()> {
        let app = TApplication::gApplication().ok_or(anyhow!("No gApplication"))?;
        let app = app.deref();

        self
            .get_canvas_impl().ok_or(anyhow!("No canvas impl"))?
            .dyn_into_ref::<TRootCanvas>().ok_or(anyhow!("No TRootCanvas"))?
            .as_TQObject()
            .connect("CloseWindow()", "TApplication", app, "Terminate()").map_err(|_| anyhow!("connect failed"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TCanvasForm {
    /// 700x500 at 10,10 (set by TStyle::SetCanvasDefH,W,X,Y)
    Form1,
    /// 500x500 at 20,20
    Form2,
    /// 500x500 at 30,30
    Form3,
    /// 500x500 at 40,40
    Form4,
    /// 500x500 at 50,50
    Form5,
}

impl TCanvasForm {
    pub fn number(&self) -> i32 {
        match self {
            TCanvasForm::Form1 => 1,
            TCanvasForm::Form2 => 2,
            TCanvasForm::Form3 => 3,
            TCanvasForm::Form4 => 4,
            TCanvasForm::Form5 => 5,
        }
    }

    pub fn from_number(number: i32) -> Option<Self> {
        match number {
            1 => Some(TCanvasForm::Form1),
            2 => Some(TCanvasForm::Form2),
            3 => Some(TCanvasForm::Form3),
            4 => Some(TCanvasForm::Form4),
            5 => Some(TCanvasForm::Form5),
            _ => None,
        }
    }
}