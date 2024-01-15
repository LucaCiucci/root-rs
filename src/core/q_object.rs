use crate::impl_utils::*;

root_object!(
    TQObject
);

impl TQObject {
    pub fn connect(&mut self, signal: &str, class: &str, receiver: &mut impl RootObject, slot: &str) -> Result<(), ()> {
        to_c_str!(signal, cl = class, slot);
        unsafe {
            let sender = self.ffi_ptr_mut();
            let receiver = receiver.ffi_ptr_mut();
            ffi_method!(TQObject::connect_sender_receiver)(
                sender,
                signal,
                cl,
                receiver as *mut _,
                slot,
            ).then(|| ()).ok_or(())
        }
    }
}