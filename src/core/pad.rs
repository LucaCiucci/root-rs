use crate::impl_utils::*;

root_object! {
    TVirtualPad(ref TObject, TAttLine, TAttFill, TAttPad, TQObject)
}

root_object! {
    TPad(TVirtualPad)
}