#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TApplication.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    RRS_STRUCT(TApplication) RRS_METHOD(TApplication, new)(const char* RRS_REF name, int argc, const char* const* RRS_REF argv) {
        int* argc_ = (int*)&argc;
        char** argv_ = (char**)argv;
        return new TApplication(name, argc_, argv_);
    }

    void RRS_METHOD(TApplication, run)(RRS_REF RRS_STRUCT(TApplication) app, bool retrn) {
        app->Run(retrn);
    }

    RRS_STRUCT(TApplication) RRS_METHOD(gApplication, get)() {
        return gApplication;
    }
}