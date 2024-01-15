use crate::impl_utils::*;

root_object! {
    TRootCanvas(TGMainFrame, ref TCanvasImp),
    ref shortcuts:
    TQObject: TGMainFrame => TGCompositeFrame => TGFrame
}