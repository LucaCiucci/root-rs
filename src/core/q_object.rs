use super::*;
use std::ffi::*;

root_object!(
    TQObject
);

impl TQObject {
    pub fn connect(&mut self, signal: &str, class: &str, receiver: &impl RootObject, slot: &str) -> Result<(), ()> {
        let sender = self.ptr();
        let signal = CString::new(signal).unwrap();
        let signal: *const c_char = signal.as_ptr() as *const _;
        let cl = CString::new(class).unwrap();
        let cl: *const c_char = cl.as_ptr() as *const _;
        let receiver = receiver.ptr();
        let slot = CString::new(slot).unwrap();
        let slot: *const c_char = slot.as_ptr() as *const _;
        unsafe {
            ffi::root_rs_TQObject__connect_sender_receiver(sender, signal, cl, receiver as *mut _, slot).then(|| ()).ok_or(())
        }
    }
}