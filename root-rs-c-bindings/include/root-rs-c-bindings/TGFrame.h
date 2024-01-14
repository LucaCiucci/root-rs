#ifdef ROOT_RS_INCLUDE_TYPES
struct TGFrame;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TGWindow) root_rs_TGFrame__as_TGWindow(RRS_STRUCT(TGFrame) RRS_VALUE frame);
RRS_STRUCT(TGFrame) root_rs_TGFrame__from_TGWindow(RRS_STRUCT(TGWindow) RRS_VALUE window);
RRS_STRUCT(TQObject) root_rs_TGFrame__as_TQObject(RRS_STRUCT(TGFrame) RRS_VALUE frame);
RRS_STRUCT(TGFrame) root_rs_TGFrame__from_TQObject(RRS_STRUCT(TQObject) RRS_VALUE obj);
void root_rs_TGFrame__delete(RRS_STRUCT(TGFrame) RRS_VALUE frame);
#endif