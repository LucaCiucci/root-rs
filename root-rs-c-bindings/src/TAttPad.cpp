
#include <root-rs-c-bindings.h>

#include <TAttPad.h>

extern "C" {
    void root_rs_TAttPad__delete(RRS_STRUCT TAttPad* RRS_VALUE att_pad) {
        delete att_pad;
    }
}