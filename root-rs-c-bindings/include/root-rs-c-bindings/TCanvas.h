#ifdef ROOT_RS_INCLUDE_TYPES
struct TCanvas;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT TPad* root_rs_TCanvas__as_TPad(RRS_STRUCT TCanvas* RRS_VALUE canvas);
RRS_STRUCT TCanvas* root_rs_TCanvas__from_TPad(RRS_STRUCT TPad* RRS_VALUE pad);
void root_rs_TCanvas__delete(RRS_STRUCT TCanvas* RRS_VALUE canvas);

RRS_STRUCT TCanvas* root_rs_TCanvas__new_build(bool build);
RRS_STRUCT TCanvas* root_rs_TCanvas__new_name_title_form(const char* RRS_CONST_REF name, const char* RRS_CONST_REF title, int form);
RRS_STRUCT TCanvas* root_rs_TCanvas__new_name_title_width_height(const char* RRS_CONST_REF name, const char* RRS_CONST_REF title, int width, int height);
RRS_STRUCT TCanvasImp* root_rs_TCanvas__GetCanvasImp(RRS_STRUCT TCanvas* RRS_MUT_REF canvas);
#endif