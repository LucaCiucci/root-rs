
#include <root-rs-c-bindings.h>

#include <TGWindow.h>

extern "C" {
    RRS_STRUCT TGObject* root_rs_TGWindow__as_TGObject(RRS_STRUCT TGWindow* RRS_VALUE window) {
        return window;
    }

    RRS_STRUCT TGWindow* root_rs_TGWindow__from_TGObject(RRS_STRUCT TGObject* RRS_VALUE obj) {
        return dynamic_cast<TGWindow*>(obj);
    }

    void root_rs_TGWindow__delete(RRS_STRUCT TGWindow* RRS_VALUE window) {
        delete window;
    }
}