#ifdef ROOT_RS_INCLUDE_TYPES
struct TQObject;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
void root_rs_TQObject__delete(RRS_STRUCT TQObject* RRS_VALUE q_object);
bool root_rs_TQObject__connect_sender_receiver(
    RRS_STRUCT TQObject* RRS_MUT_REF sender,
    const char* signal,
    const char* RRS_CONST_REF cl,
    void* RRS_MUT_REF receiver,
    const char* RRS_CONST_REF slot
);
#endif