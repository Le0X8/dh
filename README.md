<p align="center">
    <a href="https://crates.io/crates/dh"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/dh?style=for-the-badge"></a>
    <a href="https://github.com/Le0X8/dh/issues"><img alt="GitHub Issues or Pull Requests" src="https://img.shields.io/github/issues/Le0X8/dh?style=for-the-badge"></a>
    <a href="https://crates.io/crates/dh#user-content-license"><img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/dh?style=for-the-badge"></a>
</p>

<h1 align="center">
    <code>dh</code>
</h1>

<p align="center">
    <b>Data handling in Rust, made easy.</b>
</p>

## Features

- Read and write files in streams
- No unsafe code
- Support for a lot of data types (including variable length integers and custom length integers)
- Read and write u8 vectors
- std::io::Read and std::io::Write implementations for `Readable` and `Writable` (happens automatically as they extend these traits)
- Copying data from a `Readable` to a `Writable`

### Planned features

- Floating point number support
- Partial read & write access
- Temporary file storage for large data

## Installation

```bash
cargo add dh
```

## Documentation

The documentation can be found on [docs.rs](https://docs.rs/dh).

## Usage

### Simple file reading

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_r("data.txt").unwrap();
    let size = file.size().unwrap();
    assert_eq!(file.read_utf8(size), "Hello, world!\n");
}
```

### Simple file writing

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_w("data.txt").unwrap();
    file.write_utf8_at("Hello, world!\n", 0);
    file.close(); // optional, but recommended
}
```

### Open a file in read/write mode

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_rw("data.txt").unwrap();
    file.write_utf8_at("Hello, world!\n", 0);
    file.rewind();
    assert_eq!(file.read_utf8(file.size()), "Hello, world!\n");
}
```

### Read and write u8 vectors

#### Recommended: borrowing

##### Immutable borrowing

```rust
use dh::recommended::*;

fn main() {
    let mut data = vec![31u8; 1];
    let mut rw = dh::data::read_ref(&data);
    assert_eq!(rw.read_u8(), 31);
}
```

##### Mutable borrowing

```rust
use dh::recommended::*;

fn main() {
    let mut data = vec![0u8; 1];
    let mut rw = dh::data::rw_ref(&mut data);
    rw.write_u8(31);
    rw.rewind();
    assert_eq!(rw.read_u8(), 31);
}
```

#### Alternative: moving

```rust
use dh::recommended::*;

fn main() {
    let data = vec![0u8; 1];
    let mut rw = dh::data::rw(data);
    rw.write_u8(31);
    rw.rewind();
    assert_eq!(rw.read_u8(), 31);

    let data = dh::data::close(rw);
    assert_eq!(data, vec![31]);
}
```

<!--

### Read and write u8 vectors and temporarily store them in a file

```rust
use dh::{self, Readable, Writable};

fn main() {
    let data = vec![0u8; 1];
    let mut rw = dh::temp::rw(&mut data); // vector will be stored in a temporary file, reducing memory load
    rw.write_u8(31);
    rw.rewind();
    assert_eq!(rw.read_u8(), 31);
}
```

### Limit readable space

```rust
use dh::{self, Readable, Writable};

fn main() {
    let mut file = dh::file::open_r("data.txt").unwrap();
    let mut limited = file.limit(0, 5);
    assert_eq!(limited.read_utf8(5), "Hello");
}
```

-->

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
