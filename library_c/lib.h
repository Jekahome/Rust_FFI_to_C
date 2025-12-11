// File lib.h 
#ifndef LIB_H
#define LIB_H

#include <stddef.h>
#include <stdint.h>

#define FOO 123

float g_counter = 10.4f;

const char* g_msg = "Hello!";

int g_numbers[3] = {1, 2, 3};

int g_data[] = {17, 20, 0, 4};
int* g_ptr = g_data;

typedef struct {
    int x;
    float y;
} Point;

Point g_point = {.x=5, .y=78};

typedef struct {
    int arr[4];
} ArrData;

ArrData create_arr_data();

typedef struct {
    int32_t* values;
    size_t len;
} PtrToArray;

void fill_array(int* values, size_t len);
void fill_ptr_to_array(PtrToArray* p);

typedef enum {
    RED = 0,
    GREEN = 1,
    BLUE = 2
} EColor;
 
typedef union {
    int i;
    float f;
} UValue;

UValue get_uvalue(EColor);

//-------------------------------------------------------
// Callback
typedef int (*Callback)(int);
void register_cb(Callback cb);
int call_cb(int x);  // функция, которая вызывает callback
//-------------------------------------------------------

uint32_t add(uint32_t x, uint32_t y);
 
void increase_counter(float i) { g_counter += i; }

float get_counter() { return g_counter; }
 
#endif  
