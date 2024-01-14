use crate::DynCast;

use super::*;

root_object! {
    TVirtualPad: TObject, TAttLine, TAttFill, TAttPad, TQObject,
}

root_object! {
    TPad : TVirtualPad,
}