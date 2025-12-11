// File library_c/lib.c
#include "lib.h"

// C function definition. 
uint32_t add(uint32_t x, uint32_t y){
  return x + y;
}


//--------------------------------------
// Callback
static Callback g_cb = NULL;

void register_cb(Callback cb) {
    g_cb = cb;
}

int call_cb(int x) {
    if (g_cb) return g_cb(x);
    return -1;
}
//--------------------------------------
 
ArrData create_arr_data(){
    ArrData a = {.arr={10,20,30,40}};
    return a;
}

//--------------------------------------

void fill_array(int* values, size_t len) {
    for (size_t i = 0; i < len; i++) {
        values[i] = (int)(i * 10);
    }
}

void fill_ptr_to_array(PtrToArray* p){
    for (size_t i = 0; i < p->len; ++i) {
        p->values[i] = (int32_t)(i * 2);
    }
}
//--------------------------------------

UValue get_uvalue(EColor c){
    UValue u;
    if (c == RED) u.i = 123;
    if (c == GREEN) u.f = 3.14f;
    if (c == BLUE) u.i = -7;
    return u;
}