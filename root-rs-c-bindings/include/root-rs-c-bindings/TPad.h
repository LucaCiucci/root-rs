#ifdef ROOT_RS_INCLUDE_TYPES
struct TPad;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TVirtualPad) root_rs_TPad__as_TVirtualPad(RRS_STRUCT(TPad) RRS_VALUE pad);
RRS_STRUCT(TPad) root_rs_TPad__from_TVirtualPad(RRS_STRUCT(TVirtualPad) RRS_VALUE pad);
void root_rs_TPad__delete(RRS_STRUCT(TPad) RRS_VALUE pad);
#endif