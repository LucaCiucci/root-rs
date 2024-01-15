RRS_CLASS(TQObject);
//RRS_CLASS_PARENT(TObject, SOME_PARENT);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TQObject.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
bool root_rs_TQObject__connect_sender_receiver(
    RRS_STRUCT(TQObject) RRS_REF sender,
    const char* signal,
    const char* RRS_REF cl,
    void* RRS_REF receiver,
    const char* RRS_REF slot
);
#endif