#pragma once

#include <stdbool.h>

#include <root-rs-c-bindings/config.h>

#ifdef __cplusplus
    #define RRS_STRUCT(S) S*
#else
    #define RRS_STRUCT(S) struct S*
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