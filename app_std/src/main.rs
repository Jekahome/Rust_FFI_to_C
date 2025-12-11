// File: app_std/src/main.rs 
fn main() {
    let sum = lib_ffi::try_sum_vec();
    match sum {
        Ok(v) => println!("Sum: {}", v),
        Err(e) => eprintln!("Error: {}", e),
    }

  
    let s = unsafe { lib_ffi::add(2, 3) };
    println!("lib_ffi::add = {}", s);
 
    let get_g_counter: f32 = unsafe { lib_ffi::get_g_counter() };
    println!("lib_ffi::get_g_counter = {}", get_g_counter);
    unsafe {lib_ffi::increase_counter(7.0);}
    println!("lib_ffi::get_counter = {}", unsafe {lib_ffi::get_counter()});
    println!("lib_ffi::get_g_counter = {} !!! 
    переменная static mut g_counter осталась прежней, изменилась только переменная из области памяти C библиотеки.
    Доступ к ней через фцнкции. 
    \n\n", get_g_counter);

 
    let get_foo: i32 = unsafe { lib_ffi::get_FOO() };
    println!("lib_ffi::get_FOO = {:?}", get_foo);

    let g_ptr: Vec<i32> = unsafe { lib_ffi::get_ptr() };
    println!("lib_ffi::get_ptr = {:?}", g_ptr);

    let g_numbers = unsafe { lib_ffi::get_g_numbers()};
    println!("lib_ffi::g_numbers = {:?}", g_numbers);
   
    use std::ffi::CStr;
    let get_g_msg: &CStr = unsafe { lib_ffi::get_g_msg()};
    println!("lib_ffi::get_g_msg = {}", get_g_msg.to_string_lossy().into_owned());

    let get_g_msg_2: String = unsafe { lib_ffi::get_g_msg_2()};
    println!("lib_ffi::get_g_msg_2 = {}", get_g_msg_2);

    use lib_ffi::CString;
    let get_g_msg_3: CString = unsafe { lib_ffi::get_g_msg_3()};
    println!("lib_ffi::get_g_msg_3 = {}", get_g_msg_3);

    use lib_ffi::PointRs;
    let get_g_point:PointRs = unsafe { lib_ffi::get_g_point()};
    println!("lib_ffi::get_g_point x={} y={}", get_g_point.x, get_g_point.y);
    

    // Регистрируем callback -------------------------------------------------
    lib_ffi::register_callback(my_callback);

    // Вызываем C-функцию, которая вызовет callback
    let result = lib_ffi::call_callback_from_rust(5);
    println!("Result from callback: {}", result);

    // Убираем callback
    lib_ffi::unregister_callback();
    //-----------------------------------------------------------------------
    let r = lib_ffi::use_create_arr_data();
    println!("lib_ffi::use_create_arr_data = {:?}", r);
    //-----------------------------------------------------------------------
    
    let mut data = [0i32; 10].to_vec();

    lib_ffi::fill_in_slice(&mut data);

    println!("lib_ffi::fill_in_slice={:?}", data);

    let mut p = lib_ffi::PtrArrayRs::new(vec![0i32;10]);
    p.rust_alloc_and_fill();
    print!("lib_ffi::rust_alloc_and_fill=");
    p.show();
    //----------------------------------------------------------------------

    unsafe {
        // если знаем, что C отдаёт int для данного цвета:
        let val = lib_ffi::union_value::call_get_uvalue(lib_ffi::EColor::RED, true);
        println!("Got value: {:?}", val);

        // или, если хочешь читать поля из указателя:
        let u = lib_ffi::get_uvalue(lib_ffi::EColor::GREEN); // локальная копия union
        let i = lib_ffi::union_value::get_union_i(&u as *const lib_ffi::UValue);
        let f = lib_ffi::union_value::get_union_f(&u as *const lib_ffi::UValue);
        println!("i={} f={}", i, f);
    }

}

use std::os::raw::c_int;
unsafe extern "C" fn my_callback(x: c_int) -> c_int {
    println!("Callback called with {}", x);
    x + 10
}