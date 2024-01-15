RRS_CLASS(TH1F);
RRS_CLASS_PARENT(TH1F, TH1);
RRS_CLASS_PARENT(TH1F, TArrayF);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TH1F.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TH1F) RRS_METHOD(TH1F, new_range)(
    const char* RRS_REF name,
    const char* RRS_REF title,
    int n_bins_x,
    double x_low,
    double x_up
);
#endif