#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TQObject.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    bool root_rs_TQObject__connect_sender_receiver(
        RRS_STRUCT(TQObject) RRS_REF sender,
        const char* signal,
        const char* RRS_REF cl,
        void* RRS_REF receiver,
        const char* RRS_REF slot
    ) {
        return sender->Connect(signal, cl, receiver, slot);
    }
}