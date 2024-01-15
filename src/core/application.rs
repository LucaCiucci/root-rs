use crate::impl_utils::*;

root_object! {
    TApplication(TObject)
}

impl TApplication {
    pub fn new(class_name: &str) -> Ptr<Self> {
        let args = std::env::args().collect::<Vec<_>>();
        Self::new_with_args(class_name, &args)
    }

    pub fn new_with_args<Args>(class_name: &str, args: Args) -> Ptr<Self>
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

        unsafe {
            let ptr = ffi_method!(TApplication::new)(
                class_name,
                args.len() as _,
                args.as_ptr() as *const _,
            );
            Ptr::new(ptr).expect("TApplication::new failed")
        }
    }

    // TODO maybe unsafe
    pub fn gApplication() -> Option<&'static mut TApplication> {
        unsafe {
            let ptr = ffi::root_rs_gApplication__get();
            TApplication::mut_reference_from_ffi(ptr)
        }
    }

    /// Run the application and return.
    ///
    /// This is equivalent to calling `run(false)`, code
    /// after this function will be executed.
    pub fn run_return(&mut self) {
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
    pub unsafe fn run_no_return(&mut self) -> ! { // TODO maybe unsafe is not necessary, see std::mem::forget for explanation
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
    pub unsafe fn run(&mut self, ret: bool) { // TODO maybe unsafe is not necessary, see std::mem::forget for explanation
        unsafe {
            ffi::root_rs_TApplication__run(self.ffi_ptr_mut(), ret);
        }
    }
}