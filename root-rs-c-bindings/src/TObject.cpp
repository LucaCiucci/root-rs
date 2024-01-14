
#include <root-rs-c-bindings.h>

#include <TObject.h>

extern "C" {
    void root_rs_TObject__delete(RRS_STRUCT TObject* RRS_VALUE obj) {
        delete obj;
    }
}