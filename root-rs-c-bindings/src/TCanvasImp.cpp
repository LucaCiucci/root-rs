
#include <root-rs-c-bindings.h>

#include <TCanvasImp.h>

extern "C" {
    void root_rs_TCanvasImp__delete(RRS_STRUCT(TCanvasImp) RRS_VALUE canvas_imp) {
        delete canvas_imp;
    }
}