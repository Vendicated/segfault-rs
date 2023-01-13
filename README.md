# segfault

A crate that allows you to segfault for cases where you just need to

## Install

```sh
# to use in a project
$ cargo add segfault

# to use as a binary
$ cargo install segfault
```

## Usage

### Programatically

```rs
use segfault;

pub fn main() {
    segfault::segfault();
}
```

### As Command

```sh
$ segfault
```
