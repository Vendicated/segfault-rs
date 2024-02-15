// No unsafe code here!

// Uses a soundness hole: https://github.com/rust-lang/rust/issues/25860

static UNIT: &'static &'static () = &&();

fn foo<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T { v }

#[inline(never)]
fn get_dangling_ref() -> &'static Box<Box<i32>> {
    let exploit: fn(_, &Box<Box<i32>>) -> &'static Box<Box<i32>> = foo;
    exploit(UNIT, &Box::new(Box::new(42)))
}

#[inline(never)]
fn zero_memory() {
    std::hint::black_box([0; 128]);
}

pub fn segfault() -> ! {
    let dangling = get_dangling_ref();
    zero_memory();
    std::hint::black_box(***dangling);

    unreachable!();
}