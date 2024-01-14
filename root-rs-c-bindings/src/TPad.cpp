
#include <root-rs-c-bindings.h>

#include <TPad.h>

extern "C" {
    RRS_STRUCT(TVirtualPad) root_rs_TPad__as_TVirtualPad(RRS_STRUCT(TPad) RRS_VALUE pad) {
        return pad;
    }

    RRS_STRUCT(TPad) root_rs_TPad__from_TVirtualPad(RRS_STRUCT(TVirtualPad) RRS_VALUE pad) {
        return static_cast<TPad*>(pad);
    }

    void root_rs_TPad__delete(RRS_STRUCT(TPad) RRS_VALUE pad) {
        delete pad;
    }
}