#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! ffi_method {
    ($class:ident :: $method:ident) => {
        {
            use $crate::ffi;
            paste::paste! {
                // example: root_rs_TApplication__new
                ffi::[<root_rs_ $class __ $method>]
            }
        }
    };
}