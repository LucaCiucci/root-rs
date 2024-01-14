
#include <root-rs-c-bindings.h>

#include <TAttFill.h>

extern "C" {
    void root_rs_TAttFill__delete(RRS_STRUCT TAttFill* RRS_VALUE att_fill) {
        delete att_fill;
    }
}