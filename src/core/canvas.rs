use crate::impl_utils::*;

root_object! {
    TCanvas
}

impl TCanvas {
    pub fn new_build(build: bool) -> Ptr<Self> {
        unsafe {
            let ptr = ffi::root_rs_TCanvas__new_build(build);
            Ptr::new(ptr).expect("TCanvas::new_build failed")
        }
    }

    pub fn new_name_title_form(
        name: &str,
        title: &str,
        form: TCanvasForm,
    ) -> Ptr<Self> {
        to_c_str!(name, title);
        unsafe {
            let ptr = ffi::root_rs_TCanvas__new_name_title_form(
                name,
                title,
                form.number(),
            );
            Ptr::new(ptr).expect("TCanvas::new_name_title_form failed")
        }
    }

    pub fn new_with_size(
        name: &str,
        title: &str,
        width: i32,
        height: i32,
    ) -> Ptr<Self> {
        to_c_str!(name, title);
        unsafe {
            let ptr = ffi::root_rs_TCanvas__new_name_title_width_height(
                name,
                title,
                width,
                height,
            );
            Ptr::new(ptr).expect("TCanvas::new_name_title_width_height failed")
        }
    }

    pub fn get_canvas_impl(&self) -> Option<&TCanvasImp> {
        unsafe {
            TCanvasImp::reference_from_ffi(ffi::root_rs_TCanvas__GetCanvasImp(self.ffi_ptr() as *mut _))
        }
    }

    pub fn get_canvas_impl_mut(&mut self) -> Option<&mut TCanvasImp> {
        unsafe {
            TCanvasImp::mut_reference_from_ffi(ffi::root_rs_TCanvas__GetCanvasImp(self.ffi_ptr_mut()))
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

        self
            .get_canvas_impl_mut().ok_or(anyhow!("No canvas impl"))?
            .dyn_as_mut::<TRootCanvas>().ok_or(anyhow!("No TRootCanvas"))?
            .as_TQObject_mut()
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