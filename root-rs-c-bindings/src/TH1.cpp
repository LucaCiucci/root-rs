#include <root-rs-c-bindings.h>

#define ROOT_RS_INCLUDE_INCLUDES
#include <root-rs-c-bindings/TH1.h>
#undef ROOT_RS_INCLUDE_INCLUDES

extern "C" {
    bool RRS_METHOD(TH1, replace_with_sum)(
        RRS_STRUCT(TH1) RRS_REF self,
        const RRS_STRUCT(TH1) RRS_REF h1,
        const RRS_STRUCT(TH1) RRS_REF h2,
        double c1,
        double c2
    ) {
        return self->Add(h1, h2, c1, c2);
    }

    bool RRS_METHOD(TH1, add)(
        RRS_STRUCT(TH1) RRS_REF self,
        const RRS_STRUCT(TH1) RRS_REF h1,
        double c1
    ) {
        return self->Add(h1, c1);
    }

    bool RRS_METHOD(TH1, add_function)(
        RRS_STRUCT(TH1) RRS_REF self,
        RRS_STRUCT(TF1) RRS_REF f1,
        double c1,
        const char* RRS_REF option
    ) {
        return self->Add(f1, c1, option);
    }

    void RRS_METHOD(TH1, add_bin_content)(
        RRS_STRUCT(TH1) RRS_REF self,
        int bin
    ) {
        return self->AddBinContent(bin);
    }

    void RRS_METHOD(TH1, add_bin_content_w)(
        RRS_STRUCT(TH1) RRS_REF self,
        int bin,
        double w
    ) {
        return self->AddBinContent(bin, w);
    }

    double RRS_METHOD(TH1, anderson_darling_test_ad_value)(
        const RRS_STRUCT(TH1) RRS_REF self,
        const RRS_STRUCT(TH1) RRS_REF h2,
        double* RRS_REF ad_value
    ) {
        return self->AndersonDarlingTest(h2, *ad_value);
    }

    double RRS_METHOD(TH1, anderson_darling_test)(
        const RRS_STRUCT(TH1) RRS_REF self,
        const RRS_STRUCT(TH1) RRS_REF h2,
        const char* RRS_REF option
    ) {
        return self->AndersonDarlingTest(h2, option);
    }
}