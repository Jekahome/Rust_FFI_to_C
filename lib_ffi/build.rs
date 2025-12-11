// lib_ffi/build.rs

/*
Вариант 1: Классическая статическая библиотека .a (с ar вручную)

Скопировать библиотеку libfooclib.a в папку Rust-проекта, где будет линковаться

libfooclib.a   → static=fooclib

lib_ffi/build.rs:
```
fn main() {
    // Rust ищет lib + NAME + .a
    // Путь, где лежит libfooclib.a (от корня проекта)
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../library_c");

    println!("cargo:rustc-link-search=native={dir}");
    println!("cargo:rustc-link-lib=static=fooclib");// libfooclib.a -> fooclib
}
```
// Для проверки std приложения
// cargo run --package app_std 
*/
 
// ####################################################################################
/*
Вариант 2: сборка C библиотеки через crate cc который сам соберёт 

Cargo сам вызовет:
* gcc -c …
* ar rcs libfooclib.a …
* И положит libfooclib.a в target/debug/build/.../out/

Cargo.toml:
```
[build-dependencies]
cc = "1.0"
```

lib_ffi/build.rs:
```
fn main() {
    cc::Build::new()
        .file("../library_c/lib.c") // путь к твоему lib.c
        .include("../library_c")    // путь к lib.h, если нужен
        .compile("fooclib");        // создаст libfooclib.a 
 
}
```     
*/
fn main() {
    cc::Build::new()
        .file("../library_c/lib.c") // путь к твоему lib.c
        .include("../library_c")    // путь к lib.h, если нужен
        .compile("fooclib");        // создаст libfooclib.a 
 
}

 
// ####################################################################################
/*
Вариант 3: Дать Rust’у готовый .so

Rust автоматически ищет файл в виде lib<name>.so, поэтому dylib=fooclib → ищет libfooclib.so

libfooclib.so  → dylib=fooclib

> 
> Линковщик при сборке нашёл libfooclib.so по -L, но при запуске бинарника Linux ищет .so по стандартным путям (/usr/lib, /lib) или тем, что указаны в переменной LD_LIBRARY_PATH.
> 
> Сейчас libfooclib.so лежит в library_c/, а бинарник в target/debug/ — Linux её просто не видит.
> 

Вариант запуска 1:
``` 
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/jeka/Projects/Rust/Rust_FFI/std_diff_no_std/library_c
cargo run --package app_std
```

Вариант запуска 2:
* скопировать .so в папку с бинарником: cp library_c/libfooclib.so target/debug/
```
cargo run --package app_std
```

fn main() {

    let lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../library_c");

    println!("cargo:rustc-link-search=native={}", lib_dir);// относительно директории target/debug/build/lib_ffi-xxxx/out
    println!("cargo:rustc-link-lib=dylib=fooclib");// Rust ищет libfooclib.so

}
*/


// ####################################################################################
// crate bindgen
// Использование автогенерации Rust кода на основе C кода из .h 
// И вариант 2: сборка C библиотеки через crate cc который сам соберёт 
/* 
extern crate bindgen;

use std::env;
use std::path::PathBuf;
  
fn main() {
    // Путь к C исходникам
    let library_c_path = PathBuf::from("../library_c");
    let lib_c_file = library_c_path.join("lib.c");
    let lib_h_file = library_c_path.join("lib.h");

    // --- CC: компилируем C код в статическую библиотеку ---
    cc::Build::new()
        .file(&lib_c_file)
        .include(&library_c_path) // чтобы lib.c мог найти lib.h
        .compile("fooclib");      // создаст libfooclib.a

    // --- Bindgen: генерируем Rust биндинги ---
    let bindings = bindgen::Builder::default()
        .header(lib_h_file.to_str().unwrap()) // подключаем наш заголовок
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("add") // генерируем только функцию add
        .generate()
        .expect("Не удалось сгенерировать биндинги");

    // --- Записываем биндинги в $OUT_DIR/bindings.rs ---
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Не удалось записать биндинги");

    // --- Link: указываем Rust линковщику статическую библиотеку ---
    println!("cargo:rustc-link-lib=static=fooclib");
    println!("cargo:rerun-if-changed={}", lib_c_file.display());
    println!("cargo:rerun-if-changed={}", lib_h_file.display());
}
*/