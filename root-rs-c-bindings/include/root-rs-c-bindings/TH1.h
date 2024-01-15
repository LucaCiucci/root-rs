RRS_CLASS(TH1);
RRS_CLASS_PARENT(TH1, TNamed);
RRS_CLASS_PARENT(TH1, TAttLine);
RRS_CLASS_PARENT(TH1, TAttFill);
RRS_CLASS_PARENT(TH1, TAttMarker);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TH1.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
bool RRS_METHOD(TH1, replace_with_sum)(
    RRS_STRUCT(TH1) RRS_REF self,
    const RRS_STRUCT(TH1) RRS_REF h1,
    const RRS_STRUCT(TH1) RRS_REF h2,
    double c1,
    double c2
);
bool RRS_METHOD(TH1, add)(
    RRS_STRUCT(TH1) RRS_REF self,
    const RRS_STRUCT(TH1) RRS_REF h1,
    double c1
);
bool RRS_METHOD(TH1, add_function)(
    RRS_STRUCT(TH1) RRS_REF self,
    RRS_STRUCT(TF1) RRS_REF f1,
    double c1,
    const char* RRS_REF option
);
void RRS_METHOD(TH1, add_bin_content)(
    RRS_STRUCT(TH1) RRS_REF self,
    int bin
);
void RRS_METHOD(TH1, add_bin_content_w)(
    RRS_STRUCT(TH1) RRS_REF self,
    int bin,
    double w
);
double RRS_METHOD(TH1, anderson_darling_test_ad_value)(
    const RRS_STRUCT(TH1) RRS_REF self,
    const RRS_STRUCT(TH1) RRS_REF h2,
    double* RRS_REF ad_value
);
double RRS_METHOD(TH1, anderson_darling_test)(
    const RRS_STRUCT(TH1) RRS_REF self,
    const RRS_STRUCT(TH1) RRS_REF h2,
    const char* RRS_REF option
);
// TODO ...
#endif