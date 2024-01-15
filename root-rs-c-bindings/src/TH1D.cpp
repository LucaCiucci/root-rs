#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TH1D.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    RRS_STRUCT(TH1D) RRS_METHOD(TH1D, new_range)(
        const char* RRS_REF name,
        const char* RRS_REF title,
        int n_bins_x,
        double x_low,
        double x_up
    ) {
        return new TH1D(name, title, n_bins_x, x_low, x_up);
    }
}