
#include <root-rs-c-bindings.h>

#include <TCanvas.h>
#include <TPad.h>

extern "C" {
    RRS_STRUCT TPad* root_rs_TCanvas__as_TPad(RRS_STRUCT TCanvas* RRS_VALUE canvas) {
        return canvas;
    }

    RRS_STRUCT TCanvas* root_rs_TCanvas__from_TPad(RRS_STRUCT TPad* RRS_VALUE pad) {
        return static_cast<TCanvas*>(pad);
    }

    void root_rs_TCanvas__delete(RRS_STRUCT TCanvas* RRS_VALUE canvas) {
        delete canvas;
    }

    RRS_STRUCT TCanvas* root_rs_TCanvas__new_build(bool build) {
        return new TCanvas(build);
    }

    RRS_STRUCT TCanvas* root_rs_TCanvas__new_name_title_form(const char* RRS_CONST_REF name, const char* RRS_CONST_REF title, int form) {
        return new TCanvas(name, title, form);
    }

    RRS_STRUCT TCanvas* root_rs_TCanvas__new_name_title_width_height(const char* RRS_CONST_REF name, const char* RRS_CONST_REF title, int width, int height) {
        return new TCanvas(name, title, width, height);
    }

    RRS_STRUCT TCanvasImp* root_rs_TCanvas__GetCanvasImp(RRS_STRUCT TCanvas* RRS_MUT_REF canvas) {
        return canvas->GetCanvasImp();
    }
}