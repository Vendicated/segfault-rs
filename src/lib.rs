pub fn segfault() {
    unsafe { std::ptr::null_mut::<i32>().write(1) }
}
