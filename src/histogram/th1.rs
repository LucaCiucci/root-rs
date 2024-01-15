use crate::impl_utils::*;

root_object! {
    TH1(ref TNamed, TAttLine, TAttFill, TAttMarker),
    ref shortcuts:
    TObject: TNamed
}

impl TH1 {
    /// `self <- h1 * c1 + h2 * c2`
    pub fn replace_with_sum(&mut self, h1: &TH1, h2: &TH1, c1: f64, c2: f64) -> Result<()> {
        unsafe {
            ffi_method!(TH1::replace_with_sum)(
                self.ffi_ptr_mut(),
                h1.ffi_ptr(),
                h2.ffi_ptr(),
                c1,
                c2
            ).then(|| ()).ok_or(anyhow::anyhow!("TH1::replace_with_sum failed"))
        }
    }

    /// `self <- self + h1 * c1`
    pub fn add(&mut self, h1: &TH1, c1: f64) -> Result<()> {
        unsafe {
            ffi_method!(TH1::add)(
                self.ffi_ptr_mut(),
                h1.ffi_ptr(),
                c1
            ).then(|| ()).ok_or(anyhow::anyhow!("TH1::add failed"))
        }
    }

    /// `self <- self + f1 * c1`
    pub fn add_function(&mut self, f1: &mut TF1, c1: f64, option: impl Into<AddF2HOption>) -> Result<()> {
        to_c_str!(option = option.into().as_str());
        unsafe {
            ffi_method!(TH1::add_function)(
                self.ffi_ptr_mut(),
                f1.ffi_ptr_mut(),
                c1,
                option,
            ).then(|| ()).ok_or(anyhow::anyhow!("TH1::add_function failed"))
        }
    }

    /// Increment bin content by 1
    pub fn add_bin_content(&mut self, bin: isize) {
        unsafe {
            ffi_method!(TH1::add_bin_content)(
                self.ffi_ptr_mut(),
                bin as _,
            );
        }
    }

    /// Increment bin content by `w`
    pub fn add_bin_content_w(&mut self, bin: isize, w: f64) {
        unsafe {
            ffi_method!(TH1::add_bin_content_w)(
                self.ffi_ptr_mut(),
                bin as _,
                w,
            );
        }
    }

    pub fn anderson_darling_test(&self, h2: &TH1, option: impl Into<AndersonDarlingTestOption>) -> f64 {
        to_c_str!(option = option.into().as_str());
        unsafe {
            ffi_method!(TH1::anderson_darling_test)(
                self.ffi_ptr(),
                h2.ffi_ptr(),
                option,
            )
        }
    }

    pub fn anderson_darling_test_ad_value(&self, h2: &TH1) -> (f64, f64) {
        let mut ad_value = 0.;
        unsafe {
            let r = ffi_method!(TH1::anderson_darling_test_ad_value)(
                self.ffi_ptr(),
                h2.ffi_ptr(),
                &mut ad_value,
            );
            (r, ad_value)
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddF2HOption {
    #[default]
    None,
    I,
}

impl AddF2HOption {
    pub fn as_str(&self) -> &str {
        match self {
            AddF2HOption::None => "",
            AddF2HOption::I => "I",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(AddF2HOption::None),
            "I" | "i" => Some(AddF2HOption::I),
            _ => None,
        }
    }
}

impl Into<AddF2HOption> for &str {
    fn into(self) -> AddF2HOption {
        AddF2HOption::from_str(self).expect("Invalid AddF2HOption")
    }
}

impl AsRef<str> for AddF2HOption {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndersonDarlingTestOption {
    #[default]
    None,
    /// "D" Put out a line of "Debug" printout
    D,
    /// "T" Return the normalized A-D test statistic
    T,
}

impl AndersonDarlingTestOption {
    pub fn as_str(&self) -> &str {
        match self {
            AndersonDarlingTestOption::None => "",
            AndersonDarlingTestOption::D => "D",
            AndersonDarlingTestOption::T => "T",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "" => Some(AndersonDarlingTestOption::None),
            "D" | "d" => Some(AndersonDarlingTestOption::D),
            "T" | "t" => Some(AndersonDarlingTestOption::T),
            _ => None,
        }
    }
}

impl AsRef<str> for AndersonDarlingTestOption {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Into<AndersonDarlingTestOption> for &str {
    fn into(self) -> AndersonDarlingTestOption {
        AndersonDarlingTestOption::from_str(self).expect("Invalid AndersonDarlingTestOption")
    }
}