
#include <root-rs-c-bindings.h>

#include <TGObject.h>

extern "C" {
    RRS_STRUCT(TObject) root_rs_TGObject__as_TObject(RRS_STRUCT(TGObject) RRS_VALUE obj) {
        return obj;
    }

    RRS_STRUCT(TGObject) root_rs_TGObject__from_TObject(RRS_STRUCT(TObject) RRS_VALUE obj) {
        return dynamic_cast<TGObject*>(obj);
    }

    void root_rs_TGObject__delete(RRS_STRUCT(TGObject) RRS_VALUE obj) {
        delete obj;
    }
}