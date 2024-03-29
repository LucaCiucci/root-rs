
#ifdef RSS_REF
    #undef RSS_REF
#endif
#ifdef RSS_VALUE
    #undef RSS_VALUE
#endif
#ifdef RRS_UPCAST_FN_DECL
    #undef RRS_UPCAST_FN_DECL
#endif
#ifdef RRS_DOWNCAST_FN_DECL
    #undef RRS_DOWNCAST_FN_DECL
#endif
#ifdef RRS_UPCAST_FN_DEF
    #undef RRS_UPCAST_FN_DEF
#endif
#ifdef RRS_DOWNCAST_FN_DEF
    #undef RRS_DOWNCAST_FN_DEF
#endif
#ifdef RRS_DELETE_FN_DECL
    #undef RRS_DELETE_FN_DECL
#endif
#ifdef RRS_DELETE_FN_DEF
    #undef RRS_DELETE_FN_DEF
#endif
#ifdef RRS_METHOD
    #undef RRS_METHOD
#endif
#ifdef RRS_CLASS
    #undef RRS_CLASS
#endif
#ifdef RRS_CLASS_PARENT
    #undef RRS_CLASS_PARENT
#endif

#define RRS_REF
#define RRS_VALUE

#define RRS_UPCAST_FN_DECL(CLASS, PARENT) const RRS_REF RRS_STRUCT(PARENT) root_rs_ ## CLASS ## __as_ ## PARENT(const RRS_REF RRS_STRUCT(CLASS) cl)
#define RRS_DOWNCAST_FN_DECL(CLASS, PARENT) const RRS_REF RRS_STRUCT(CLASS) root_rs_ ## CLASS ## __from_ ## PARENT(const RRS_REF RRS_STRUCT(PARENT) parent)
#define RRS_UPCAST_FN_DEF(CLASS, PARENT) RRS_UPCAST_FN_DECL(CLASS, PARENT) { return cl; }
#define RRS_DOWNCAST_FN_DEF(CLASS, PARENT) RRS_DOWNCAST_FN_DECL(CLASS, PARENT) { return dynamic_cast<const CLASS*>(parent); }

#define RRS_DELETE_FN_DECL(TYPE) void root_rs_ ## TYPE ## __delete(const RRS_VALUE RRS_STRUCT(TYPE) obj)
#define RRS_DELETE_FN_DEF(TYPE) RRS_DELETE_FN_DECL(TYPE) { delete obj; }

#define RRS_METHOD(CLASS, METHOD) root_rs_ ## CLASS ## __ ## METHOD

#ifdef ROOT_RS_INCLUDE_TYPES
    #define RRS_CLASS(CLASS) \
        struct CLASS; \
        RRS_DELETE_FN_DECL(CLASS);
#else
    #define RRS_CLASS(CLASS)
#endif

#ifdef ROOT_RS_INCLUDE_FUNCTIONS
    #define RRS_CLASS_PARENT(CLASS, PARENT) \
        RRS_UPCAST_FN_DECL(CLASS, PARENT); \
        RRS_DOWNCAST_FN_DECL(CLASS, PARENT);
#elif defined(RRS_CLASSES_BASIC_IMPLEMENTATION)
    #undef RRS_CLASS
    #define RRS_CLASS(CLASS) RRS_DELETE_FN_DEF(CLASS);
    #define RRS_CLASS_PARENT(CLASS, PARENT) \
        RRS_UPCAST_FN_DEF(CLASS, PARENT); \
        RRS_DOWNCAST_FN_DEF(CLASS, PARENT);
#else
    #define RRS_CLASS_PARENT(CLASS, PARENT)
#endif


#include "./TObject.h"
#include "./TQObject.h"
#include "./TGObject.h"
#include "./TApplication.h"
#include "./TVirtualPad.h"
#include "./TAttLine.h"
#include "./TAttFill.h"
#include "./TAttPad.h"
#include "./TAttMarker.h"
#include "./TPad.h"
#include "./TCanvas.h"
#include "./TGWindow.h"
#include "./TGFrame.h"
#include "./TCanvasImp.h"
#include "./TGCompositeFrame.h"
#include "./TGMainFrame.h"
#include "./TRootCanvas.h"
#include "./TNamed.h"
#include "./TH1.h"
#include "./TF1.h"
#include "./TH1F.h"
#include "./TH1D.h"
#include "./TArray.h"
#include "./TArrayF.h"
#include "./TArrayD.h"