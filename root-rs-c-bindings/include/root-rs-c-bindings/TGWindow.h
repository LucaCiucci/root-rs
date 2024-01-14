#ifdef ROOT_RS_INCLUDE_TYPES
struct TGWindow;
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
RRS_STRUCT(TGObject) root_rs_TGWindow__as_TGObject(RRS_STRUCT(TGWindow) RRS_VALUE window);
RRS_STRUCT(TGWindow) root_rs_TGWindow__from_TGObject(RRS_STRUCT(TGObject) RRS_VALUE obj);
void root_rs_TGWindow__delete(RRS_STRUCT(TGWindow) RRS_VALUE window);
#endif