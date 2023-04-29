pub fn segfault() -> ! {
    unsafe {
        libc::raise(11);
        libc::raise(11);
        std::hint::unreachable_unchecked()
    }
}
