use std::ops::RangeInclusive;

use crate::impl_utils::*;

root_object! {
    TH1D(ref TH1, TArrayD),
    ref shortcuts:
    TNamed: TH1,
    TAttLine: TH1,
    TAttFill: TH1,
    TAttMarker: TH1,
    TObject: TH1 => TNamed,
    TArray: TArrayD,
}

impl TH1D {
    pub fn new_range(name: &str, title: &str, n_bins: usize, range: RangeInclusive<f64>) -> Ptr<Self> {
        to_c_str!(name, title);
        unsafe {
            let ptr = ffi_method!(TH1D::new_range)(
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