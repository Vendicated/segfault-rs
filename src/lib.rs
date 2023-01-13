pub fn segfault(ptr: *mut i32) {
    if !ptr.is_null() {
        unsafe { ptr.write(1) }
    } else {
        println!("Error: null pointer");
    }
}
