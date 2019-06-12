use libc::{c_char};
use std::ffi::{CStr, CString};
use zecpaperlib::{paper, pdf};

#[no_mangle]
pub extern fn rust_generate_wallet(testnet: bool, count: u32) -> *mut c_char {
    let c_str = CString::new(paper::generate_wallet(testnet, false, count)).unwrap();
    return c_str.into_raw();
}

#[no_mangle]
pub extern fn rust_save_to_pdf(i_addrs: *const c_char, i_filename: *const c_char) {
    let c_addrs = unsafe {
        assert!(!i_addrs.is_null());

        CStr::from_ptr(i_addrs)
    };

    let c_filename = unsafe {
        assert!(!i_filename.is_null());

        CStr::from_ptr(i_filename)
    };

    let addresses = c_addrs.to_str().unwrap();
    let filename  = c_filename.to_str().unwrap();

    pdf::save_to_pdf(addresses, filename);
}

#[no_mangle]
pub extern fn rust_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

// #[no_mangle]
// pub extern fn double_input(input: i32) -> i32 {
//     input * 2
// }

// #[no_mangle]
// pub extern fn say_hello() -> *mut c_char {
//     let mut hello = String::from("Hello World");
//     hello.push_str(", ZecWallet!");

//     let c_str_song = CString::new(hello).unwrap();
//     c_str_song.into_raw()
// }

// #[no_mangle]
// pub extern fn free_str(s: *mut c_char) {
//     let s = unsafe {
//         if s.is_null() { return }
//         CString::from_raw(s)
//     };

//     println!("Freeing {:?}", s);
// }