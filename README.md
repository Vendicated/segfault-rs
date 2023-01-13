# segfault

A crate that allows you to segfault for cases where you just need to

## Install

```sh
# to use in a project
$ cargo add segfault

# to use as a command
$ cargo install segfault
```

## Usage

### Programatically

```rs
use segfault;

pub fn main() {
    let ptr: *mut i32 = std::ptr::null_mut();
    segfault::segfault_safe(ptr);
}
```

### As Command

```sh
$ segfault
```

## Support This Project

To help me make more useful crates, consider [sponsoring me](https://github.com/sponsors/Vendicated)

Proudly powered by JetBrains
