RRS_CLASS(TApplication);
RRS_CLASS_PARENT(TApplication, TObject);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TApplication.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TApplication) RRS_METHOD(TApplication, new)(const char* RRS_REF name, int argc, const char* const* RRS_REF argv);
void RRS_METHOD(TApplication, run)(RRS_REF RRS_STRUCT(TApplication) app, bool retrn);
RRS_STRUCT(TApplication) RRS_METHOD(gApplication, get)();
#endif