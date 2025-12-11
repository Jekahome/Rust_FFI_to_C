// File: lib_ffi/src/lib.rs

// This brings the generated FFI declarations into our crate.
// The code will be located at $OUT_DIR/bindings.rs
// We must disable some lints that are commonly triggered by C code.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(not(feature = "std"))]
use alloc::format;

#[cfg(feature = "std")]
use std::vec::Vec;

/// Fallible allocation
pub fn try_build_vec() -> Result<Vec<u32>, String> {
    let mut v: Vec<u32> = Vec::new();

    #[cfg(not(feature = "std"))]
    let required_size = 4;

    #[cfg(not(feature = "std"))]
    v.try_reserve(required_size)
        .map_err(|_e| format!("Failed to allocate {} items!", required_size))?;

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    Ok(v)
}

pub fn try_sum_vec() -> Result<usize, String> {
    let v = try_build_vec()?;
    Ok(v.iter().copied().sum::<u32>() as usize)
}


/*

В std режиме (по умолчанию):
* используется std::vec::Vec
* #![no_std] не включается
* Сборка с бинырными файлами: cargo build
* Сборка только lib: cargo build --lib


В no_std режиме:
* подключается extern crate alloc;
* Vec берётся из alloc::vec::Vec
* Сборка: 
    * rustup target add thumbv6m-none-eabi --toolchain
    * cargo build --lib --no-default-features --target thumbv6m-none-eabi

*/
 
// ######################################################################### 
// FFI
// #########################################################################
/*

## Для std приложения

app_std собирается под x86_64-unknown-linux-gnu и C-шный libfooclib.a тоже был x86_64 → они совместимы


## Для no_std приложения

Они не могут использовать динамическую библиотеку .so.
Только статическую (.a).

То есть:
* app_no_std → работает только с libfooclib.a
* app2_no_std → тоже только с .a

К тому же библиотеку C придется собирать используя целевой компилятор системы.
А если собрать C код как ELF под x86_64 Linux, а Rust пытается собрать бинарь под ARM Cortex-M, то есть bare-metal, это не позволит слинковаться этим файлам.

Надо установить компилятор
sudo apt install gcc-arm-none-eabi

Собери C библиотеку под ARM Cortex-M
* Например для thumbv6m:
    arm-none-eabi-gcc -c lib.c -o lib.o -mcpu=cortex-m0 -mthumb
    arm-none-eabi-ar rcs libfooclib.a lib.o
* Для thumbv7em:
    arm-none-eabi-gcc -c lib.c -o lib.o -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard
    arm-none-eabi-ar rcs libfooclib.a lib.o

Теперь Rust сможет собрать no_std проект без ошибок
cargo build --package app2_no_std \
  --no-default-features \
  --target thumbv7em-none-eabihf

*/

/*
Декларация помечена как extern "C" чтобы указать, что код для функции будет предоставлен внешней библиотекой C.   
Маркер extern "C" также автоматически помечает функцию как #[no_mangle]


Таблица: Указатели C → Rust

| C                | Rust                                         |
| ---------------- | -------------------------------------------- |
| `int*`           | `*mut c_int` / `*const c_int`                |
| `void*`          | `*mut c_void` / `*const c_void`              |
| `char*`          | `*mut c_char` / `*const c_char`              |
| `float*`         | `*mut f32` / `*const f32`                    |
| `int (*cb)(int)` | `Option<unsafe extern "C" fn(c_int)->c_int>` |


*/
 

#[cfg(not(feature = "std"))]
use core::ffi::{c_int, c_char};

// use core::ffi::c_size_t;

#[cfg(feature = "std")]
use std::os::raw::{c_int, c_char, c_float, c_void};

 

extern "C" {
    pub fn add(x: c_int, y: c_int) -> c_int;

    // В FFI глобальные переменные C не гарантируют синхронизации с Rust static mut, если мы не вызываем их через функции C. 
    // Rust просто видит адрес, а C может оптимизировать доступ по-своему.

    pub static mut g_counter: c_float;
    pub fn increase_counter(i: c_float);
    pub fn get_counter() -> c_float;


    pub static g_msg: *const c_char;

    pub static g_numbers: [c_int; 3];

    pub static g_ptr: *const c_int;

    // Константы С "#define FOO 123" не существуют в бинарнике, их нет как символов, линковщик ничего о них не знает.
    // Поэтому Rust их не увидит напрямую.
    // bindgen генерирует:
    // pub const FOO: ::std::os::raw::c_int = 123;
    // Если не используем bindgen → константы надо написать вручную:
    // pub const FOO: i32 = 123;
   
    pub fn register_cb(cb: Option<unsafe extern "C" fn(c_int) -> c_int>);
    pub fn call_cb(x: c_int) -> c_int;

    pub static mut g_point: Point;

    pub fn create_arr_data() -> ArrData;

    pub fn fill_array(ptr: *mut c_int, len: usize);

    pub fn fill_ptr_to_array(p: *mut PtrToArray);

    pub fn get_uvalue(c: EColor) -> UValue;


}

pub const FOO: i32 = 123;
 
#[repr(C)]
pub struct Point {
    pub x: c_int,
    pub y: f32,
}


#[repr(C)]
pub struct ArrData {
    pub arr: [c_int; 4],
}

#[repr(C)]
pub struct PtrToArray {
    pub values: *mut c_int,
    pub len: usize,
}

#[repr(C)]
pub enum EColor {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}

#[repr(C)]
pub union UValue {
    pub i: c_int,
    pub f: f32,
}

 
//---------------------------------------------------
// Функции для демонстрации данных

pub unsafe fn get_g_counter() -> c_float{
    g_counter
} 

pub unsafe fn get_FOO() -> c_int{
    FOO
} 

pub unsafe fn get_ptr() -> Vec<i32>{
    let slice = std::slice::from_raw_parts(g_ptr, 4);
    let v = slice.to_vec();
    v
}

pub unsafe fn get_g_numbers() -> Vec<i32>{
    let v = g_numbers.to_vec();
    v
}

/*
Чего нет в no_std
* CString
* OsStr, OsString
* String, если нет alloc

Но можно включить аллокатор, и тогда появится String и возможность делать свои аналоги CString.
*/
#[cfg(feature = "std")]
use std::ffi::CStr;

pub unsafe fn get_g_msg() -> &'static CStr{
    let s: &CStr = CStr::from_ptr(g_msg);
    s
}

pub unsafe fn get_g_msg_2() -> String{
    let c_str: &CStr = CStr::from_ptr(g_msg);
    c_str.to_string_lossy().into_owned()
}

pub use my_csting::CString;
pub mod my_csting{
    use super::c_char;
    pub struct CString {
        inner: Vec<u8>,
    }

    impl CString {
        pub fn new<S: AsRef<[u8]>>(s: S) -> Self {
            let mut v = s.as_ref().to_vec();
            v.push(0);
            Self { inner: v }
        }

        pub fn as_ptr(&self) -> *const c_char {
            self.inner.as_ptr() as *const c_char
        }

        /// Rust-friendly UTF-8 string (or lossy fallback)
        pub fn to_rust_string(&self) -> String {
            let slice = &self.inner[..self.inner.len() - 1]; // exclude '\0'

            match core::str::from_utf8(slice) {
                Ok(s) => String::from(s),
                Err(_) => slice.iter().map(|&b| b as char).collect(),
            }
        }  
    }
    use core::fmt;
    impl fmt::Display for CString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.to_rust_string())
        }
    }
    impl fmt::Debug for CString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("CString(")?;
            f.write_str(&self.to_rust_string())?;
            f.write_str(")")
        }
    }    
}

pub unsafe fn get_g_msg_3() -> CString {
    let cstr: &CStr = CStr::from_ptr(g_msg);
    let bytes = cstr.to_bytes();
    // Создаём наш CString
    CString::new(bytes)
}


// register_callback -------------------------------------------------- 

pub fn register_callback(cb: unsafe extern "C" fn(c_int) -> c_int) {
    unsafe { register_cb(Some(cb)) }
}

pub fn unregister_callback() {
    unsafe { register_cb(None) }
}

pub fn call_callback_from_rust(x: c_int) -> c_int {
    unsafe { call_cb(x) }
}

// Point ----------------------------------------------------------------

pub use point_rs::PointRs;
pub mod point_rs{

    use super::Point;

    pub struct PointRs {
        pub x: i32,
        pub y: f32,
    }

    impl From<Point> for PointRs {
        fn from(p: Point) -> Self {
            Self { x: p.x, y: p.y }
        }
    }

    use core::ptr;
    pub unsafe fn get_point(p: *const Point) -> PointRs {
        let val: Point = ptr::read(p);   // корректно копируем Point
        PointRs::from(val)
    }
}

pub unsafe fn get_g_point() -> PointRs{
    let val: Point = core::ptr::read(&raw const g_point); // копируем g_point
    PointRs::from(val)
}

// ArrData-----------------------------------------------------------
pub fn use_create_arr_data()->[i32;4]{
    let arr_data = unsafe{create_arr_data()};
    let l:&[i32; 4] = arrdata_to_slice(&arr_data);
    let l:[i32; 4] = unsafe{ get_arr_data(&arr_data as *const ArrData)};
    l
}

pub fn arrdata_to_slice(a: &ArrData) -> &[i32; 4] {
    // безопасно: repr(C) гарантирует layout
    unsafe { &*(&a.arr as *const [c_int; 4] as *const [i32; 4]) }
}

pub unsafe fn get_arr_data(a: *const ArrData) -> [i32; 4] {
    (*a).arr.map(|x| x as i32)
}

// PtrToArray --------------------------------------------------------
pub use ptr_array::PtrArrayRs;
pub mod ptr_array{
    use super::{PtrToArray, fill_ptr_to_array};
 
    pub struct PtrArrayRs{
        values: Vec<i32>,
    }

    impl PtrArrayRs{
        pub fn new(values: Vec<i32>) -> Self{
            Self{values}
        }

        pub fn rust_alloc_and_fill(&mut self) {

            let mut pa = PtrToArray {
                values: self.values.as_mut_ptr(),
                len: self.values.len(),
            };

            unsafe {
                fill_ptr_to_array(&mut pa);
            }
 
        }
        pub fn show(&self){
            println!("{:?}",self.values);
        }
    }

    impl From<PtrToArray> for PtrArrayRs {
        fn from(s: PtrToArray) -> Self {
            PtrArrayRs{values: unsafe{ ptr_array_as_slice(&s).to_vec()} }
        }
    }

    pub unsafe fn ptr_array_as_slice(a: &PtrToArray) -> &[i32] {
        core::slice::from_raw_parts(a.values as *const i32, a.len)
    }

    pub unsafe fn ptr_array_as_slice_mut(a: &mut PtrToArray) -> &mut [i32] {
        core::slice::from_raw_parts_mut(a.values as *mut i32, a.len)
    }

    pub unsafe fn get_ptr_array(a: *const PtrToArray) -> &'static [i32] {
        let a = &*a;
        core::slice::from_raw_parts(a.values, a.len)
    }

}

pub fn fill_in_slice(buf: &mut Vec<i32>) {
    unsafe {
        fill_array(buf.as_mut_ptr(), buf.len());
    }
}

// EColor -------------------------------------------------------------
pub use enum_color::{ColorRs, get_color};
pub mod enum_color{
    use super::EColor;

    #[derive(Debug, Clone, Copy)]
    pub enum ColorRs {
        Red,
        Green,
        Blue,
    }

    impl From<EColor> for ColorRs {
        fn from(c: EColor) -> Self {
            match c {
                EColor::RED => ColorRs::Red,
                EColor::GREEN => ColorRs::Green,
                EColor::BLUE => ColorRs::Blue,
            }
        }
    }
    impl From<ColorRs> for EColor {
        fn from(c: ColorRs) -> Self {
            match c {
                ColorRs::Red   => EColor::RED,
                ColorRs::Green => EColor::GREEN,
                ColorRs::Blue  => EColor::BLUE,
            }
        }
    }

    pub unsafe fn get_color(c: EColor) -> ColorRs {
        c.into()
    }    
}

// UValue -------------------------------------------------------------
pub mod union_value {
    use super::UValue;
    use std::os::raw::c_int;

    /// Rust-удобный вариант union'а
    #[derive(Debug)]
    pub enum UValueRs {
        Int(i32),
        Float(f32),
    }

    /// Методы для чтения union (unsafe — чтение union'а небезопасно)
    impl UValue {
        pub unsafe fn as_int(&self) -> i32 {
            // читаем поле i, приводим к i32
            self.i as i32
        }
        pub unsafe fn as_float(&self) -> f32 {
            self.f
        }
    }

    /// Конвертировать union в Rust-enum по флагу is_int
    /// is_int == true  -> читаем как Int
    /// is_int == false -> читаем как Float
    pub unsafe fn uvalue_to_rs(u: &UValue, is_int: bool) -> UValueRs {
        if is_int {
            UValueRs::Int(u.as_int())
        } else {
            UValueRs::Float(u.as_float())
        }
    }

    /// Небольшие удобные функции чтения по указателю (если у тебя есть *const UValue)
    pub unsafe fn get_union_i(u: *const UValue) -> i32 {
        if u.is_null() { return 0; }
        (*u).as_int()
    }

    pub unsafe fn get_union_f(u: *const UValue) -> f32 {
        if u.is_null() { return 0.0; }
        (*u).as_float()
    }

    /// Вызываем C get_uvalue и возвращаем UValueRs, определяя формат по флагу is_int.
    /// Возвращаем Rust-тип, безопаснее чем раздавать raw union наружу.
    pub unsafe fn call_get_uvalue(color: super::EColor, is_int: bool) -> UValueRs {
        // вызов внешней C-функции:
        let u: UValue = crate::get_uvalue(color);
        // читаем локальную копию u
        uvalue_to_rs(&u, is_int)
    }
}
 

// #########################################################################
// Использование помошника crate bindgen
/*
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


Подключит файл:
 /home/jeka/Projects/Rust/Rust_FFI/std_diff_no_std/target/debug/build/lib_ffi-50670de34cf469d1/out/bindings.rs:210:5
...
unsafe extern "C" {
    pub fn add(x: u32, y: u32) -> u32;
}

*/
 

