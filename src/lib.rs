#![allow(non_snake_case)]

pub mod ffi;

fn _tmp() {
    let _ = ffi::RRS_ROOT_DOC_VERSION;
}

pub trait RootObject {
    type FFIType;

    unsafe fn ffi_ptr(&self) -> *const Self::FFIType;

    unsafe fn ffi_ptr_mut(&mut self) -> *mut Self::FFIType;

    unsafe fn reference_from_ffi<'a>(ptr: *mut Self::FFIType) -> Option<&'a Self>;

    unsafe fn mut_reference_from_ffi<'a>(ptr: *mut Self::FFIType) -> Option<&'a mut Self>;

    unsafe fn delete(ptr: *mut Self::FFIType);

    fn dyn_as<To: RootObject>(&self) -> Option<&To>
    where
        Self: DynCast<To>
    {
        unsafe {
            let ptr = self.ffi_ptr() as *mut _;
            let to_ptr = Self::dyn_ptr_cast(ptr);

            if to_ptr.is_null() {
                None
            } else {
                Some(&*(to_ptr as *const _))
            }
        }
    }

    fn dyn_as_mut<To: RootObject>(&mut self) -> Option<&mut To>
    where
        Self: DynCast<To>
    {
        unsafe {
            let ptr = self.ffi_ptr_mut();
            let to_ptr = Self::dyn_ptr_cast(ptr);

            if to_ptr.is_null() {
                None
            } else {
                Some(&mut *(to_ptr as *mut _))
            }
        }
    }
}

pub trait DynCast<Into: RootObject>: RootObject {
    unsafe fn dyn_ptr_cast(ptr: *mut Self::FFIType) -> *mut Into::FFIType;
}

pub struct Ptr<T: RootObject>(*mut T);

impl<T: RootObject> Ptr<T> {
    pub unsafe fn new(ptr: *mut T::FFIType) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr as *mut T))
        }
    }

    pub unsafe fn leak(self) -> *mut T {
        let ptr = self.0;
        std::mem::forget(self);
        ptr
    }

    pub unsafe fn leak_ffi(self) -> *mut T::FFIType {
        self.leak() as *mut _
    }

    pub unsafe fn dyn_into<To: RootObject>(self) -> Result<Ptr<To>, Ptr<T>>
    where
        T: DynCast<To>,
    {
        unsafe {
            let t_ptr = self.ffi_ptr() as *mut _;
            let to_ptr = T::dyn_ptr_cast(t_ptr);

            if to_ptr.is_null() {
                Err(self)
            } else {
                std::mem::forget(self);
                Ok(Ptr::new(to_ptr).unwrap())
            }
        }
    }
}

impl<T: RootObject> Drop for Ptr<T> {
    fn drop(&mut self) {
        assert!(self.0.is_null() == false);
        unsafe {
            T::delete((*self.0).ffi_ptr_mut());
        }
    }
}

impl<T: RootObject> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        debug_assert!(self.0.is_null() == false);
        unsafe {
            &*self.0
        }
    }
}

impl<T: RootObject> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(self.0.is_null() == false);
        unsafe { &mut *self.0 }
    }
}

macro_rules! root_object {
    (
        $(#[$meta:meta])*
        $name:ident $(( $($parents:tt)* ))?
        $( // shortcuts
            , ref shortcuts:
            $(
                $shortcut_base:ident: $($shortcut_path:ident)=>+
            ),* $(,)?
        )?
    ) => {
        #[doc = $crate::root_object_synopsys!($name)]
        $(#[$meta])*
        #[doc = $crate::root_object_doc!($name)]
        #[doc = "\n\n # Inheritance\n"]
        $(#[doc = $crate::impl_root_object_base_classes!("doc" $name( $($parents)* ))])?
        $(#[doc = $crate::impl_root_object_shortcuts!("doc" $name: $($shortcut_base: $($shortcut_path)=>+),*)])?
        #[doc = concat!(
            "\n\n![inherit graph](https://root.cern/doc/v", std::env!("RRS_ROOT_DOC_VERSION_STR"), "/class", stringify!($name), "__inherit__graph_org.svg",
            ")\n",
        )]
        pub struct $name(std::marker::PhantomData<*mut ()>);

        $crate::impl_root_object_trait!($name);
        $(
            $crate::impl_root_object_base_classes!($name( $($parents)* ));
        )?
        $(
            $crate::impl_root_object_shortcuts!($name: $($shortcut_base: $($shortcut_path)=>+),*);
        )?

        impl $name {
        }
    };
}

macro_rules! root_object_synopsys {
    ($name:ident) => {
        concat!(
            "Root [`", stringify!($name), "`](",
            "https://root.cern/doc/v", std::env!("RRS_ROOT_DOC_VERSION_STR"), "/class", stringify!($name), ".html",
            ") class\n\n",
        )
    }
}
pub(crate) use root_object_synopsys;

macro_rules! root_object_doc {
    ($name:ident) => {
        concat!(
            "\n# Root documentation\n",
            "<",
            "https://root.cern/doc/v", std::env!("RRS_ROOT_DOC_VERSION_STR"), "/class", stringify!($name), ".html",
            ">\n",
        )
    }
}
pub(crate) use root_object_doc;

macro_rules! impl_root_object_trait {
    ($name:ident) => {
        impl $crate::RootObject for $name {
            type FFIType = $crate::ffi::$name;

            unsafe fn ffi_ptr(&self) -> *const Self::FFIType {
                self as *const _ as *const _
            }
        
            unsafe fn ffi_ptr_mut(&mut self) -> *mut Self::FFIType {
                self as *mut _ as *mut _
            }

            unsafe fn reference_from_ffi<'a>(ptr: *mut Self::FFIType) -> Option<&'a Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(&*(ptr as *const _))
                }
            }

            unsafe fn mut_reference_from_ffi<'a>(ptr: *mut Self::FFIType) -> Option<&'a mut Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(&mut *(ptr as *mut _))
                }
            }

            unsafe fn delete(ptr: *mut Self::FFIType) {
                use $crate::ffi;
                paste::paste! {
                    ffi::[<root_rs_ $name __ delete>](ptr);
                }
            }
        }
    };
}
pub(crate) use impl_root_object_trait;

macro_rules! impl_root_object_base_classes {
    ($name:ident( $base:ident $(,)? )) => {
        $crate::impl_root_object_base_class!($name : ref $base);
    };
    ($name:ident( $($first:ident $($second:ident)?),* $(,)? )) => {
        $(
            $crate::impl_root_object_base_class!($name : $first $($second)?);
        )*
    };
    ("doc" $name:ident( $base:ident $(,)? )) => {
        concat!(
            "Parent [`", $crate::impl_root_object_base_class!("doc" $name : ref $base), "`]\n\n",
        )
    };
    ("doc" $name:ident( $($first:ident $($second:ident)?),* $(,)? )) => {
        concat!(
            "Parents:\n",
            $(
                concat!(
                    "- ",
                    $crate::impl_root_object_base_class!("doc" $name : $first $($second)?),
                    "\n",
                ),
            )*
            "\n\n",
        )
    };
}
pub(crate) use impl_root_object_base_classes;

macro_rules! impl_root_object_base_class {
    ($name:ident : $base:ident) => {
        impl $name {
            paste::paste! {
                #[doc = concat!(
                    "Reference to the parent [`", stringify!($base), "`] class\n\n",
                )]
                pub fn [<as_ $base>]<'s>(&'s self) -> &'s $base {
                    self.dyn_as().unwrap()
                }
                #[doc = concat!(
                    "Mutable reference to the parent [`", stringify!($base), "`] class\n\n",
                )]
                pub fn [<as_ $base _mut>]<'s>(&'s mut self) -> &'s mut $base {
                    self.dyn_as_mut().unwrap()
                }
            }
        }

        paste::paste! {
            impl_dyn_cast! {
                $name => $base : [<root_rs_ $name __ as _ $base>];
                $base => $name : [<root_rs_ $name __ from _ $base>];
            }
        }
    };
    ($name:ident : ref $base:ident) => {
        $crate::impl_root_object_base_class!($name : $base);

        impl Deref for $name {
            type Target = $base;

            fn deref(&self) -> &Self::Target {
                self.dyn_as().unwrap()
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.dyn_as_mut().unwrap()
            }
        }
    };
    ("doc" $name:ident : $base:ident) => {
        concat!(
            "[`",
            stringify!($base),
            "`]: can be accessed with [`as_", stringify!($base), "`](", stringify!($name), "::as_", stringify!($base), ") or the [`DynCast` trait](crate::DynCast)",
        )
    };
    ("doc" $name:ident : ref $base:ident) => {
        concat!(
            "*",
            $crate::impl_root_object_base_class!("doc" $name : $base),
            ", also accessible with [`Deref`] (coercion)",
        )
    };
}
pub(crate) use impl_root_object_base_class;

macro_rules! impl_root_object_shortcuts {
    (
        $base:ident:
        $(
            $shortcut_base:ident: $($shortcut_path:ident)=>+
        ),* $(,)?
    ) => {
        $(
            $crate::impl_root_object_shortcut!($base: $shortcut_base $($shortcut_path)=>+);
        )*
    };
    (
        "doc"
        $base:ident:
        $(
            $shortcut_base:ident: $($shortcut_path:ident)=>+
        ),* $(,)?
    ) => {
        concat!(
            "Shortcuts for higher level parent classes:\n",
            $(
                concat!(
                    "- ",
                    $crate::impl_root_object_shortcut!("doc" $base: $shortcut_base $($shortcut_path)=>+),
                    "\n",
                ),
            )*
            "\n\n",
        )
    };
}
pub(crate) use impl_root_object_shortcuts;

macro_rules! impl_root_object_shortcut {
    ($name:ident: $shortcut_base:ident $($shortcut_path:ident)=>+) => {
        impl $name {
            paste::paste! {
                #[doc = concat!(
                    "Reference to the parent [`", stringify!($shortcut_base), "`] class\n\n",
                )]
                /// # Remarks
                #[doc = concat!(
                    "[`", stringify!($shortcut_base), "`] is not a direct parent, but this is a shortcut fot the following hierarchy path:  \n",
                    "[`", stringify!($name), "`]->",
                    $(
                        "[`", stringify!($shortcut_path), "`] -> ",
                    )*
                    "[`", stringify!($shortcut_base), "`]\n\n",
                )]
                pub fn [<as_ $shortcut_base>]<'s>(&'s self) -> &'s $shortcut_base {
                    self
                    $(
                        . [<as_ $shortcut_path>]()
                    )+
                    . [<as_ $shortcut_base>]()
                }
                pub fn [<as_ $shortcut_base _mut>]<'s>(&'s mut self) -> &'s mut $shortcut_base {
                    self
                    $(
                        . [<as_ $shortcut_path _mut>]()
                    )+
                    . [<as_ $shortcut_base _mut>]()
                }
            }
        }

        impl DynCast<$shortcut_base> for $name {
            unsafe fn dyn_ptr_cast(ptr: *mut Self::FFIType) -> *mut <$shortcut_base as RootObject>::FFIType {
                paste::paste! {
                    let r = Self::mut_reference_from_ffi(ptr).unwrap();
                    $(
                        let r = r.[<as_ $shortcut_path _mut>]();
                    )*
                    let r = r.[<as_ $shortcut_base _mut>]();
                    r.ffi_ptr_mut()
                }
            }
        }
    };
    ($name:ident: ref $shortcut_base:ident $($shortcut_path:ident)=>+) => {
        $crate::impl_root_object_shortcut!($name: $shortcut_base $($shortcut_path)=>+);

        impl Deref for $name {
            type Target = $shortcut_base;

            fn deref(&self) -> &Self::Target {
                self.dyn_as().unwrap()
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.dyn_as_mut().unwrap()
            }
        }
    };
    ("doc" $name:ident: $shortcut_base:ident $($shortcut_path:ident)=>+) => {
        concat!(
            "[`",
            stringify!($shortcut_base),
            "`]: shortcut for ",
            $("[`", stringify!($shortcut_path), "`] ->",)* "[`", stringify!($shortcut_base), "`]",
            ", can be accessed with [`as_", stringify!($shortcut_base), "`](", stringify!($name), "::as_", stringify!($shortcut_base), ") or the [`DynCast<", stringify!($shortcut_base), ">` trait](crate::DynCast)",
        )
    };
    ("doc" $name:ident: ref $shortcut_base:ident $($shortcut_path:ident)=>+) => {
        concat!(
            "*",
            $crate::impl_root_object_shortcut!("doc" $name: $shortcut_base $($shortcut_path)=>+),
            ", also accessible with [`Deref`] (coercion)",
        )
    };
}
pub(crate) use impl_root_object_shortcut;

//$(
//    $shortcut_base:ident: $($shortcut_path:ident)=>+
//),* $(,)?

pub(crate) use root_object;

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
            unsafe fn dyn_ptr_cast(ptr: *mut Self::FFIType) -> *mut <$to as $crate::RootObject>::FFIType {
                $crate::ffi::$f(ptr) as *mut _
            }
            //fn dyn_cast(self) -> Option<$to> {
            //    unsafe {
            //        let ptr = ffi::$f(self.0);
            //        <$to as $crate::RootObject>::from_ptr(ptr as *mut _)
            //    }
            //}
            //fn dyn_cast_ref<'s>(&'s self) -> Option<$crate::RootRef<'s, $to>> {
            //    unsafe {
            //        let ptr = ffi::$f(self.0);
            //        $crate::RootRef::new(ptr as *mut _)
            //    }
            //}
        }
    };
    ($($from:ty => $to:ty : $f:ident);+ $(;)?) => {
        $($crate::impl_dyn_cast!($from => $to : $f);)+
    };
}

use std::ops::{Deref, DerefMut};

pub(crate) use impl_dyn_cast;

pub mod core;
pub mod gui;
pub mod histogram;
pub mod function;

pub mod all_modules {
    pub use crate::core::*;
    pub use crate::gui::*;
    pub use crate::histogram::*;
    pub use crate::function::*;
}

pub(crate) mod impl_utils {
    pub use crate::*;
    pub use crate::all_modules::*;
    pub use anyhow::{Result, anyhow};
    pub use std::ffi::*;
}