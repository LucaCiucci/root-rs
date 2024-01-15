RRS_CLASS(TObject);
//RRS_CLASS_PARENT(TObject, SOME_PARENT);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TObject.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
void RRS_METHOD(TObject, draw)(
    RRS_STRUCT(TObject) RRS_REF self,
    const char* RRS_REF option
);
#endif