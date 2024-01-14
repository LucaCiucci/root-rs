#ifdef ROOT_RS_INCLUDE_TYPES
struct TVirtualPad;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TObject) root_rs_TVirtualPad__as_TObject(RRS_STRUCT(TVirtualPad) RRS_MUT_REF pad);
RRS_STRUCT(TAttLine) root_rs_TVirtualPad__as_TAttLine(RRS_STRUCT(TVirtualPad) RRS_MUT_REF pad);
RRS_STRUCT(TAttFill) root_rs_TVirtualPad__as_TAttFill(RRS_STRUCT(TVirtualPad) RRS_MUT_REF pad);
RRS_STRUCT(TAttPad) root_rs_TVirtualPad__as_TAttPad(RRS_STRUCT(TVirtualPad) RRS_MUT_REF pad);
RRS_STRUCT(TQObject) root_rs_TVirtualPad__as_TQObject(RRS_STRUCT(TVirtualPad) RRS_MUT_REF pad);
RRS_STRUCT(TVirtualPad) root_rs_TVirtualPad__from_TObject(RRS_STRUCT(TObject) RRS_MUT_REF obj);
RRS_STRUCT(TVirtualPad) root_rs_TVirtualPad__from_TAttLine(RRS_STRUCT(TAttLine) RRS_MUT_REF att_line);
RRS_STRUCT(TVirtualPad) root_rs_TVirtualPad__from_TAttFill(RRS_STRUCT(TAttFill) RRS_MUT_REF att_fill);
RRS_STRUCT(TVirtualPad) root_rs_TVirtualPad__from_TAttPad(RRS_STRUCT(TAttPad) RRS_MUT_REF att_pad);
RRS_STRUCT(TVirtualPad) root_rs_TVirtualPad__from_TQObject(RRS_STRUCT(TQObject) RRS_MUT_REF q_object);
void root_rs_TVirtualPad__delete(RRS_STRUCT(TVirtualPad) RRS_VALUE pad);
#endif