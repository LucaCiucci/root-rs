
#include <root-rs-c-bindings.h>

#include <TAttLine.h>

extern "C" {
    void root_rs_TAttLine__delete(RRS_STRUCT TAttLine* RRS_VALUE att_line) {
        delete att_line;
    }
}