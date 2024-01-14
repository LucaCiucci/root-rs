#pragma once

#include <stdbool.h>

#include <root-rs-c-bindings/info.h>

#define RRS_MUT_REF
#define RRS_CONST_REF
#define RRS_VALUE

#ifdef __cplusplus
    #define RRS_STRUCT
#else
    #define RRS_STRUCT struct
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define ROOT_RS_INCLUDE_TYPES
#include "root-rs-c-bindings/_all.h"
#undef ROOT_RS_INCLUDE_TYPES

#define ROOT_RS_INCLUDE_FUNCTIONS
#include "root-rs-c-bindings/_all.h"
#undef ROOT_RS_INCLUDE_FUNCTIONS

#ifdef __cplusplus
}
#endif