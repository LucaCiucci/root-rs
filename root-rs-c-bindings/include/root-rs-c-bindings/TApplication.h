RRS_CLASS(TApplication);
RRS_CLASS_PARENT(TApplication, TObject);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TApplication.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT TApplication* RRS_METHOD_NAME(TApplication, new)(const char* RRS_CONST_REF name, int argc, const char* const* RRS_CONST_REF argv);
void RRS_METHOD_NAME(TApplication, run)(RRS_STRUCT TApplication* RRS_MUT_REF app, bool retrn);
RRS_STRUCT TApplication* RRS_METHOD_NAME(gApplication, get)();
#endif