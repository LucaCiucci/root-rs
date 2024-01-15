use crate::impl_utils::*;

//pub struct TObject(pub(crate) *mut ffi::TObject);

root_object!(
    TObject
);

impl TObject {
    pub fn draw_with_option(&mut self, option: &str) {
        to_c_str!(option);
        unsafe {
            ffi_method!(TObject::draw)(self.ffi_ptr_mut(), option);
        }
    }

    pub fn draw(&mut self) {
        self.draw_with_option("");
    }
}