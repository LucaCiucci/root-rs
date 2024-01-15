#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TObject.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    void RRS_METHOD(TObject, draw)(
        RRS_STRUCT(TObject) RRS_REF self,
        const char* RRS_REF option
    ) {
        self->Draw(option);
    }
}