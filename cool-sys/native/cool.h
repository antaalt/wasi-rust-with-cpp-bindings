#pragma once

// This is required to avoid linking issues as we bind interface as C in rust FFI
#ifdef __cplusplus
extern "C" {
#endif

struct CoolStruct {
    int x;
    int y;
};
void cool_function(int i, char c, CoolStruct* cs);
void test();


#ifdef __cplusplus
}
#endif