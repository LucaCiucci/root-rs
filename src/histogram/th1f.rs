use std::ops::RangeInclusive;

use crate::impl_utils::*;

root_object! {
    TH1F(ref TH1, TArrayF),
    ref shortcuts:
    TNamed: TH1,
    TAttLine: TH1,
    TAttFill: TH1,
    TAttMarker: TH1,
    TObject: TH1 => TNamed,
    TArray: TArrayF,
}

impl TH1F {
    pub fn new_range(name: &str, title: &str, n_bins: usize, range: RangeInclusive<f64>) -> Ptr<Self> {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let title = CString::new(title).unwrap();
        let title = title.as_ptr();
        unsafe {
            let ptr = ffi_method!(TH1F::new_range)(
                name,
                title,
                n_bins as i32,
                *range.start() as f64,
                *range.end() as f64,
            );
            Ptr::new(ptr).expect("TH1F::new_range failed")
        }
    }
}