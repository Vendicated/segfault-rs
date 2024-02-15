pub fn segfault() -> ! {
    unsafe {
        std::ptr::null_mut::<i32>().write(1);
        libc::raise(11);
        std::hint::unreachable_unchecked()
    }
}

mod safe;

pub fn segfault_safe() -> ! {
    safe::segfault()
}