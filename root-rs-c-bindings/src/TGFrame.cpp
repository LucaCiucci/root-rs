
#include <root-rs-c-bindings.h>

#include <TGFrame.h>

extern "C" {
    RRS_STRUCT TGWindow* root_rs_TGFrame__as_TGWindow(RRS_STRUCT TGFrame* RRS_VALUE frame) {
        return frame;
    }

    RRS_STRUCT TGFrame* root_rs_TGFrame__from_TGWindow(RRS_STRUCT TGWindow* RRS_VALUE window) {
        return dynamic_cast<TGFrame*>(window);
    }

    RRS_STRUCT TQObject* root_rs_TGFrame__as_TQObject(RRS_STRUCT TGFrame* RRS_VALUE frame) {
        return frame;
    }

    RRS_STRUCT TGFrame* root_rs_TGFrame__from_TQObject(RRS_STRUCT TQObject* RRS_VALUE obj) {
        return dynamic_cast<TGFrame*>(obj);
    }

    void root_rs_TGFrame__delete(RRS_STRUCT TGFrame* RRS_VALUE frame) {
        delete frame;
    }
}