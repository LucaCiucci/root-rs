
use crate::ffi_method;

use super::*;
use std::ffi::*;

root_object! {
    TApplication : TObject
}

impl TApplication {
    pub fn new(class_name: &str) -> Self {
        let args = std::env::args().collect::<Vec<_>>();
        Self::new_with_args(class_name, &args)
    }

    pub fn new_with_args<Args>(class_name: &str, args: Args) -> Self
    where
        Args: IntoIterator,
        Args::Item: AsRef<str>,
    {
        let class_name = CString::new(class_name).unwrap();
        let class_name: *const c_char = class_name.as_ptr() as *const _;

        let args = args
            .into_iter()
            .map(|s| CString::new(s.as_ref()).unwrap())
            .collect::<Vec<_>>();
        let args: Vec<*const c_char> = args.iter().map(|s| s.as_ptr() as *const _).collect();

        let ptr = unsafe {
            ffi_method!(TApplication::new)(
                class_name,
                args.len() as _,
                args.as_ptr() as *const _,
            )
        };

        TApplication(ptr)
    }

    pub fn gApplication() -> Option<RootRef<'static, TApplication>> {
        unsafe {
            let ptr = ffi::root_rs_gApplication__get();
            if ptr.is_null() {
                None
            } else {
                Some(RootRef::new(ptr).unwrap())
            }
        }
    }

    /// Run the application and return.
    ///
    /// This is equivalent to calling `run(false)`, code
    /// after this function will be executed.
    pub fn run_return(&self) {
        unsafe {
            self.run(true);
        }
    }

    /// Run the application and never return.
    ///
    /// This is equivalent to calling `run(true)`, code
    /// after this function will not be executed.
    ///
    /// # Remarks
    /// This function is unsafe because it will never return, RAII
    /// might not be executed properly.
    pub unsafe fn run_no_return(&self) -> ! {
        unsafe {
            self.run(false);
        }
        unreachable!("TApplication::Run() returned even though it should not have");
    }

    /// Run the application.
    ///
    /// # Arguments
    /// * `ret` - If true, the application will return after the event loop is terminated.
    ///
    /// # Notes
    /// - This function is unsafe, use [run_return] or [run_no_return] instead of this function.
    ///
    /// [run_return]:TApplication::run_return
    /// [run_no_return]:TApplication::run_no_return
    pub unsafe fn run(&self, ret: bool) {
        unsafe {
            ffi::root_rs_TApplication__run(self.0, ret);
        }
    }
}