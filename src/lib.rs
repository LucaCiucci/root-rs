#![allow(non_snake_case)]

pub mod ffi;

fn tmp() {
    ffi::RRS_ROOT_VERSION;
}

pub trait RootObject: Sized {
    type FFIType;

    fn from_ptr(ptr: *mut Self::FFIType) -> Option<Self>;

    /// returns the FFI pointer without dropping
    fn unwrap_ptr(self) -> *mut Self::FFIType;

    fn ptr(&self) -> *mut Self::FFIType;
}

pub struct RootRef<'a, T: RootObject>(Option<T>, PhantomData<&'a()>);

impl<'a, T: RootObject> RootRef<'a, T> {
    fn new(ptr: *mut T::FFIType) -> Option<Self> {
        T::from_ptr(ptr).map(|t| Self(Some(t), PhantomData))
    }
}

impl<'a, T: RootObject> Drop for RootRef<'a, T> {
    fn drop(&mut self) {
        // this is just a reference, no need to delete
        let _ = self.0.take().unwrap().unwrap_ptr();
    }
}

impl<'a, T: RootObject> Deref for RootRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<'a, T: RootObject> DerefMut for RootRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

macro_rules! root_object {
    (
        $(#[$meta:meta])*
        $name:ident $(: $($base:ident),+ $(,)?)?
        $( // shortcuts
            ref
            $(
                $shortcut_base:ident: $($shortcut_path:ident)=>+
            )* $(,)?
        )?
    ) => {
        $(#[$meta])*
        #[doc = "# Root documentation\n"]
        #[doc = concat!(
            "[`", stringify!($name), "`](",
            "https://root.cern/doc/v", std::env!("RRS_ROOT_DOC_VERSION_STR"), "/class", stringify!($name), ".html",
            "): <",
            "https://root.cern/doc/v", std::env!("RRS_ROOT_DOC_VERSION_STR"), "/class", stringify!($name), ".html",
            ">\n",
        )]
        pub struct $name(pub(crate) *mut $crate::ffi::$name);

        impl Drop for $name {
            fn drop(&mut self) {
                use paste::paste;
                use $crate::ffi;
                unsafe {
                    paste!(ffi::[<root_rs_ $name __ delete>](self.0));
                }
            }
        }

        impl $name {
            pub fn as_ref<'s>(&'s self) -> $crate::RootRef<'s, $name> {
                $crate::RootRef::new(self.0).unwrap()
            }

            $(
                $(
                    paste::paste! {
                        pub fn [<as_ $base>]<'s>(&'s self) -> $crate::RootRef<'s, $base> {
                            self.dyn_cast_ref().unwrap()
                        }
                        pub fn [<as_ $base _mut>]<'s>(&'s mut self) -> $crate::RootRef<'s, $base> {
                            self.dyn_cast_ref().unwrap()
                        }
                    }
                )*
            )?

            $( // TODO do the same for dyn_cast
                $(
                    paste::paste! {
                        pub fn [<as_ $shortcut_base>]<'s>(&'s self) -> $crate::RootRef<'s, $shortcut_base> {
                            use std::ops::Deref;
                            let ptr = self
                                $(
                                    .dyn_into_ref::<$shortcut_path>().unwrap().deref()
                                )+
                                .dyn_into_ref::<$shortcut_base>().unwrap().deref().ptr();
                            $crate::RootRef::new(ptr).unwrap()
                        }
                        pub fn [<as_ $shortcut_base _mut>]<'s>(&'s mut self) -> $crate::RootRef<'s, $shortcut_base> {
                            let ptr = self.[<as _ $shortcut_base>]().ptr();
                            $crate::RootRef::new(ptr).unwrap()
                        }
                    }
                )*
            )?

            pub fn dyn_into<To: $crate::RootObject>(self) -> Option<To>
            where
                Self: $crate::DynCast<To>,
            {
                self.dyn_cast()
            }

            pub fn dyn_into_ref<'s, To: $crate::RootObject>(&'s self) -> Option<$crate::RootRef<'s, To>>
            where
                Self: $crate::DynCast<To>,
            {
                self.dyn_cast_ref()
            }
        }

        impl $crate::RootObject for $name {
            type FFIType = $crate::ffi::$name;

            fn from_ptr(ptr: *mut Self::FFIType) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self(ptr))
                }
            }

            fn unwrap_ptr(self) -> *mut Self::FFIType {
                let ptr = self.0;
                std::mem::forget(self);
                ptr
            }

            fn ptr(&self) -> *mut Self::FFIType {
                self.0
            }
        }

        $(
            $(
                paste::paste! {
                    impl_dyn_cast! {
                        $name => $base : [<root_rs_ $name __ as _ $base>];
                        $base => $name : [<root_rs_ $name __ from _ $base>];
                    }
                }
            )*
        )?
    };
}

pub(crate) use root_object;

pub trait DynCast<To: RootObject> {
    fn dyn_cast(self) -> Option<To>;
    fn dyn_cast_ref<'s>(&'s self) -> Option<RootRef<'s, To>>;
}

//pub trait DynCastInto {
//    fn dyn_cast_into<To: RootObject>(self) -> Option<To> where Self: DynCast<To>;
//    fn dyn_cast_ref_into<'s, To: RootObject>(&'s self) -> Option<RootRef<'s, To>> where Self: DynCast<To>;
//}

//impl<From> DynCastInto for From {
//    fn dyn_cast_into<To: RootObject>(self) -> Option<To> where Self: DynCast<To> {
//        self.dyn_cast()
//    }
//    fn dyn_cast_ref_into<'s, To: RootObject>(&'s self) -> Option<RootRef<'s, To>> where Self: DynCast<To> {
//        self.dyn_cast_ref()
//    }
//}

macro_rules! impl_dyn_cast {
    ($from:ty => $to:ty : $f:ident) => {
        impl $crate::DynCast<$to> for $from {
            fn dyn_cast(self) -> Option<$to> {
                unsafe {
                    let ptr = ffi::$f(self.0);
                    <$to as $crate::RootObject>::from_ptr(ptr)
                }
            }
            fn dyn_cast_ref<'s>(&'s self) -> Option<$crate::RootRef<'s, $to>> {
                unsafe {
                    let ptr = ffi::$f(self.0);
                    $crate::RootRef::new(ptr)
                }
            }
        }
    };
    ($($from:ty => $to:ty : $f:ident);+ $(;)?) => {
        $($crate::impl_dyn_cast!($from => $to : $f);)+
    };
}

use std::{ops::{Deref, DerefMut}, marker::PhantomData};

pub(crate) use impl_dyn_cast;

pub mod core;
pub mod gui;
pub mod histogram;