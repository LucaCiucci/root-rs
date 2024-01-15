#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TH1F.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    RRS_STRUCT(TH1F) RRS_METHOD(TH1F, new_range)(
        const char* RRS_REF name,
        const char* RRS_REF title,
        int n_bins_x,
        double x_low,
        double x_up
    ) {
        return new TH1F(name, title, n_bins_x, x_low, x_up);
    }
}