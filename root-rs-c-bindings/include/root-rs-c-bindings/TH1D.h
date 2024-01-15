RRS_CLASS(TH1D);
RRS_CLASS_PARENT(TH1D, TH1);
RRS_CLASS_PARENT(TH1D, TArrayD);

#ifdef ROOT_RS_INCLUDE_INCLUDES
#include <TH1D.h>
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TH1D) RRS_METHOD(TH1D, new_range)(
    const char* RRS_REF name,
    const char* RRS_REF title,
    int n_bins_x,
    double x_low,
    double x_up
);
#endif