# Rust FFI

Язык C имеет четко определенный интерфейс двоичных приложений (ABI), что делает его общим знаменателем для взаимодействия между многими языками.

Важно знать оба направления **Rust to C and C to Rust**.
Но начинать проще и полезнее всего с: **Rust to C** (использование C-библиотек в Rust). 
Это наиболее распространённый кейс: Rust-код использует существующие C-библиотеки.

**Что нужно знать**
* Что такое ABI
* Что такое C ABI
* `#[repr(C)]` и layout типов
* `extern "C"` для функций
* Разница между `unsafe` и гарантиями Rust
* Как работает linking (static / dynamic)


## 2. Rust → C

### Понимание типов

* Сопоставление типов Rust ↔ C
  (`i32` ↔ `int32_t`, `*mut T`, `bool`, `usize`, `c_char`)

### Работа со строками

* `CString`, `CStr`
* UTF-8 vs ASCII / null-terminated

### Структуры и enum’ы

* `#[repr(C)] struct …`
* Почему нельзя передавать обычные Rust enums

### Библиотеки и инструменты

* `bindgen` для генерации Rust-обёрток над C API
* `cc` crate для сборки C-кода вместе с Rust

![Rust to C](https://github.com/Jekahome/Rust_FFI_to_C/blob/master/FFI.png)

---

## 3. C → Rust

### Экспорт Rust-функций

* `#[no_mangle]`
* `pub extern "C" fn …`
* Работа с безопасными API поверх unsafe ядра

### Управление памятью

* Кто освобождает память?
* Rust-аллокатор vs malloc/free
* Создание функций вида:

  ```rust
  void* foo_create();
  void foo_destroy(void*);
  ```

### Экспорт структур

* Только `#[repr(C)]`
* Недопустимость Vec, String, Box<T> как есть

### Создание динамических библиотек

* crate-type = ["cdylib", "staticlib"]
* Подготовка заголовочного файла (.h)

---

## 4. Практические темы

* Передача callback-функций Rust → C → Rust
* Multithreading и FFI
* Panic across FFI boundaries (нельзя превышать границы!)
* Вызов C++ кода через C-shims
* FFI-safe типы
* FFI error handling (возврат кодов, errno, error structs)

---



## no_std

Это имеет смысл только в том случае, если все ящики, от которых вы зависите также no_std.

В std есть две большие части:
* базовая библиотека языка → core
* аллоцируемые структуры → alloc
* взаимодействие с ОС → std (filesystem, threads, sockets, mutex, stdout, таймеры)

Но когда мы отключаем std т.е. используем no_std то alloc нам нужно явно подключить (core остается и так)


Rust поставляется со стандартной библиотекой, которая называется `std`, который включает код для широкого спектра распространённых задач: от стандартных структур данных до работы с сетями, от поддержки многопоточности до файлового ввода-вывода. Для удобства некоторые элементы из std автоматически импортируются в вашу программу через [::prelude](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents) набор общих `use` операторов, которые делают доступными распространенные типы без необходимости использования их полных имен (например, `Vec` вместо std::vec::Vec).

Атрибут `#![no_std]` уровня ящика в верхней части src/lib.rs

Rust поддерживает конфигурации без полной стандартной библиотеки std, что позволяет использовать его в системах, не имеющих полноценной операционной системы (например, без файловой системы, без сети). Rust также поддерживает сборку кода для сред, где невозможно предоставить полную стандартную библиотеку, таких как загрузчики, прошивки и встраиваемые платформы в целом. 

При std компилятор вставляет импорты основных примитивов по умолчанию.

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
```

Что бы отключить стандартные импорты используйте

```rust
#![no_implicit_prelude]
```

Но для no_std необходимо явно включать нужные импорты.

## Проверка зависимостей на no_std

Так как компилятор не сообщает что std а что no_std мы можем явно задать цель сборки в которой отсутвует std и тогда подтягиваемые зависимсоти с std сломают компиляцию, что даст нам понимание от чего избавится.

Мы собираем проект под таргет, в котором вообще нет std, например: thumbv6m-none-eabi (ARM Cortex-M0, популярный no_std target)

1. Добавить локально (и в CI) таргет для cross-compile

```
rustup target add thumbv6m-none-eabi
```

2. Команда для проверки no_std сборки

Если у вас библиотека (lib.rs):

```
cargo build --lib --target thumbv6m-none-eabi
```

Если у вас бинарник, то лучше отключить std через features:

Cargo features аддитивные — они только добавляют возможности, но никогда не должны выключать. 
Поэтому опция — добавляет std, а не отключает его.  (Cargo объединяет features, выполняя логическое ИЛИ поэтому если один выключит а другой включи то по итогу получится features включена)


По умолчанию включена фича std
* Если std включена → включаются dep1/std и dep2/std
* Если std выключена → твой crate работает в no_std

Очень важно: Фича std только добавляет функциональность, а не отключает.

Cargo.toml:
```toml
[features]
default = ["std"]
std = ["dep1/std", "dep2/std"] 
```

И в коде:
```
#![cfg_attr(not(feature = "std"), no_std)]
```

Сборка:
```
cargo build --lib --no-default-features --target thumbv6m-none-eabi
```

Вариант через cargo tree:
```bash
if cargo tree -i std | grep -q "std"; then
    echo "❌ ERROR: std detected in dependency tree!"
    exit 1
else
    echo "✔ OK: no std in dependencies"
fi
```

**Нельзя так:**

```toml
[features]
no_std = []
```

```rust
#[cfg(feature = "no_std")]
fn something_without_std() { ... }

#[cfg(not(feature = "no_std"))]
fn something_with_std() { ... }
```

Потому что один пользователь включает no_std, другой нет → итог = включено → ломается.


### core

Библиотека core содержит код который не выделяет память в heap. Поэтому структур данных Vec, Map, Sets там нет.

Даже при разработке для самых ограниченных платформ многие фундаментальные типы из стандартной библиотеки остаются доступными. Например, Option и Result. Они по-прежнему доступны через `core::{Option, Result, Iterator, slice}`

Типы из core доступны для всех программ Rust автоматически. Однако, как правило, их необходимо явно `use` указывать в `no_std` среде, посколькуstd `prelude` отсутствует.

### alloc

Для использования heap в среде no_std должен быть аллокатор (например linked_list_allocator, buddy_system_allocator, jemalloc, ваш собственный…). Нужно включить crate alloc и тогда будут доступны структуры данных для работы с памятью heap 

```rust
alloc::boxed::Box<T>
alloc::rc::Rc<T>
alloc::sync::Arc<T>
alloc::vec::Vec<T>
alloc::string::String
alloc::format!
alloc::collections::BTreeMap<K, V>
alloc::collections::BTreeSet<T>
```

Тип std::vec::Vec на самом деле это alloc::vec::Vec

Отсутствуют коллекции HashMap и HashSet так как для генерации хешей нужны возможности ОС. Но есть BTreeMap и BTreeSet.

Отсутствует структура синхронизации std::sync::Mutex. Для многопоточного кода в no_std используют crate [spin](https://docs.rs/spin/)

Если вы это сделали — у вас появляется heap и выделение памяти в no_std

 

```rust
#![no_std]
extern crate alloc;

use alloc::vec::Vec;

#[global_allocator]
static ALLOC: MyAllocator = MyAllocator::new();

```
Каждый вызов Vec::push() может вызвать аллокацию.
Но Rust не предполагает что выделение памяти в heap может дать ошибку, т.е. Rust предполагает, что аллокатор не может провалиться.
Нет способа обработать failure и продолжить выполнение, как в C (malloc → NULL). И это поведение Rust не подходит для embedded, kernel, или ограниченных систем.

Rust начал добавлять альтернативы, которые возвращают Result:
* Vec::try_reserve(n) — резервирует память, возвращает `Result<(), AllocError>`
* Box::try_new(value) — возвращает `Result<Box<T>, AllocError>` (nightly)

Пока нет полноценного Vec::try_push(), поэтому приходится сначала резервировать память, а потом делать push().

```rust
fn try_build_a_vec() -> Result<Vec<u8>, String> {
    let mut v = Vec::new();

    let required_size = 4;
    // Сначала резервируем память через try_reserve
    v.try_reserve(required_size)
        .map_err(|_e| format!("Failed to allocate {} items!", required_size))?;

    // Если аллокация успешна → безопасно делаем push
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    Ok(v)
}
```

Или можно не использовать heap и отключить выделение памяти. В некоторых системах (например Linux kernel, embedded):

Можно отключить глобальную обработку OOM через `no_global_oom_handling`.

Тогда любая попытка infallible аллокации станет ошибкой компиляции/сборки, если она случайно появится.


## Проблема сборки под no_std

Rust должен знать, на какой процессор будет компилировать, для этого нужно выбрать цель, по умолчанию используется **x86_64-unknown-linux-gnu** в Linux. Но когда мы отключаем std **#![no_std]** то цель нужно явно указать.

Цель нужна для правильной компиляции core/alloc под конкретный процессор.

Аллокатор linked_list_allocator для блокировки (spinlock) использует spinning_top который нуждается в атомарных инструкциях (compare_exchange, compare_exchange_weak) на AtomicBool.
 
Но цель сборки **thumbv6m-none-eabi** не поддерживают атомарные инструкции.

Цель сборки **thumbv7em-none-eabihf**  поддерживают атомарные инструкции и всё работает.

**Для симуляции no_std-бинарника, таргет без атомарных инструкций thumbv6m-none-eabi для проверки отсутствия std зависимостей:**

(нужен свой аллокатор)

```

rustup target add thumbv6m-none-eabi
cargo build --package app_no_std --no-default-features --target thumbv6m-none-eabi

```

**Для no_std-бинарника без проверки отсутствия std зависимостей, таргет с атомарными инструкциями thumbv7em-none-eabihf:**

(используем библиотечный аллокатор linked_list_allocator)

```
 
rustup target add thumbv7em-none-eabihf
cargo build --package app2_no_std --no-default-features --target thumbv7em-none-eabihf 

```



---

## FFI


[Список поддерживаемых Rust ABI](https://doc.rust-lang.org/reference/items/external-blocks.html#abi)

**Чтобы Rust линковал C библиотеку (через build.rs):**

* Вариант 1: Классическая статическая библиотека .a (с ar вручную)
* Вариант 2: сборка C библиотеки через crate cc который сам соберёт 
* Вариант 3: Дать Rust’у готовую динамическую библиотеку .so



**Name mangling (искажение имен)**

Это касается C++ в котором есть возможность использовать перегрузку, когда имена функций одинаковые. Тогда компоновщик применяет искажение имен функций и что бы их увидеть используют инструмент **nm**

```cpp
ns1::add(int,int) -> __ZN3ns13addEii
ns1::add(long,long) -> __ZN3ns13addExx
```

```
nm ffi-lib.o | grep add  # what the linker sees for C
0000000000000000 T _add

nm ffi-cpp-lib.o | grep add  # what the linker sees for C++
0000000000000000 T __ZN3ns13addEii

nm ffi-cpp-lib.o | grep add | c++filt  # use c++filt
0000000000000000 T ns1::add(int, int)
```

Rust FFI с C: используем extern "C" + #[no_mangle] → символы без mangling, linker просто соединяет, типы не проверяет.

То есть когда используешь Rust с C через FFI, важно следить, чтобы сигнатуры совпадали, иначе компоновщик пропустит ошибку, а краш будет уже во время работы.



**Типы**

На стороне Rust если есть выбор то, есть модуль std::os::raw, который определяет базовые C-подобные типы для FFI. Использование std::os::raw::{c_int, c_long, c_char, c_void} гарантирует, что Rust будет использовать тот же размер и представление, что и int в C на той же платформе.

А если есть выбор какой тип данных использовать в языке C то лучше использовать типы из библиотеки stdint.h, например uint32_t, int64_t и т.д., мы точно знаем размер типа: 32 бита, 64 бита и т.п.


crate libc нужен не для базовых типов которые есть в std::os::raw, а для большего покрытия C API и системных констант
* Структуры и типы, которых нет в std::os::raw
* Константы и макросы C
* Системные функции (libc даёт почти все POSIX/Unix функции: mmap, open, read, write, fork, getpid и т.д)

crate bindgen инструмент, который автоматически генерирует Rust FFI-код на основе C-заголовков (.h). Используется в build.rs


[Выравнивание структуры #[repr(C)]](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)

Для строк [alloc::ffi::CString](https://doc.rust-lang.org/alloc/ffi/struct.CString.html) и [alloc::ffi::CStr](https://doc.rust-lang.org/core/ffi/struct.CStr.html)

 
---

## Links

**Официальные материалы Rust**

* 1. [Beyond Standard Rust](https://www.lurklurk.org/effective-rust/beyond-std.html)

* The Rustonomicon – FFI:
  [nomicon/ffi](https://doc.rust-lang.org/nomicon/ffi.html)
 
* Reference — extern and ABI
  [reference/external-blocks](https://doc.rust-lang.org/reference/items/external-blocks.html)

**C/Rust interaction**

* Rust FFI Omnibus
  [jakegoulding.com/rust-ffi-omnibus](https://jakegoulding.com/rust-ffi-omnibus/)

* [Rust FFI (Why Some Projects Use Multiple Programming Languages)](https://youtu.be/XJC5WB2Bwrc?si=xNsSubwz8Frlx9vm&t=783)
 
[CMake_CPP_to_RUST_examples](https://github.com/Jekahome/CMake_CPP_to_RUST_examples)

📌 [Working with C unions in Rust FFI](https://hermanradtke.com/2016/03/17/unions-rust-ffi.html/)
