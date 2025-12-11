

# Сборка

## Для std-бинарника:

```
cargo run --package app_std

```

## Проблема сборки под no_std

Rust должен знать, на какой процессор будет компилировать, для этого нужно выбрать цель, по умолчанию используется **x86_64-unknown-linux-gnu** в Linux. Но когда мы отключаем std **#![no_std]** то цель нужно явно указать.

Цель нужна для правильной компиляции core/alloc под конкретный процессор.

Аллокатор linked_list_allocator для блокировки (spinlock) использует spinning_top который нуждается в атомарных инструкциях (compare_exchange, compare_exchange_weak) на AtomicBool.
 
Но цель сборки **thumbv6m-none-eabi** не поддерживают атомарные инструкции.

Цель сборки **thumbv7em-none-eabihf**  поддерживают атомарные инструкции и всё работает.

## Для симуляции no_std-бинарника, таргет без атомарных инструкций thumbv6m-none-eabi для проверки отсутвия std зависимостей:
(нужен свой аллокатор)

```
rustup target add thumbv6m-none-eabi
cargo build --package app_no_std --no-default-features --target thumbv6m-none-eabi

```

## Для no_std-бинарника без проверки отсутвия std зависимостей, таргет с атомарными инструкциями thumbv7em-none-eabihf:
(используем библиотечный аллокатор linked_list_allocator)

```
 
rustup target add thumbv7em-none-eabihf
cargo build --package app2_no_std --no-default-features --target thumbv7em-none-eabihf 


```
