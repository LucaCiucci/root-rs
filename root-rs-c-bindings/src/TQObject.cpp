
#include <root-rs-c-bindings.h>

#include <TQObject.h>

extern "C" {
    void root_rs_TQObject__delete(RRS_STRUCT TQObject* RRS_VALUE q_object) {
        delete q_object;
    }

    bool root_rs_TQObject__connect_sender_receiver(
        RRS_STRUCT TQObject* RRS_MUT_REF sender,
        const char* signal,
        const char* RRS_CONST_REF cl,
        void* RRS_MUT_REF receiver,
        const char* RRS_CONST_REF slot
    ) {
        return sender->Connect(signal, cl, receiver, slot);
    }
}