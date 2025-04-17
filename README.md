<p align="center">
    <a href="https://crates.io/crates/dh"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/dh?style=for-the-badge"></a>
    <a href="https://github.com/Le0X8/dh/issues"><img alt="GitHub Issues or Pull Requests" src="https://img.shields.io/github/issues/Le0X8/dh?style=for-the-badge"></a>
    <a href="https://github.com/Le0X8/dh/actions/workflows/ci.yml"><img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/Le0X8/dh/ci.yml?style=for-the-badge&label=ci"></a>
    <!-- <a href="https://crates.io/crates/dh#user-content-license"><img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/dh?style=for-the-badge"></a> -->
</p>

<h1 align="center">
    <code>dh</code>
</h1>

<p align="center">
    <b>Data handling in Rust, made easy.</b>
</p>

## Features

- Read and write files in streams
- Support for a lot of data types (including custom length integers)
- Read and write u8 vectors
- std::io::Read and std::io::Write implementations for `ReadVal` and `WriteVal` (happens automatically as they extend these traits)
- Copying data from `ReadVal` to `WriteVal`
- Partial read & write access with `limit`
- Floating point number support

### Planned features

- Zero-cost cloning
- Zero-cost subarray clones
- Reading and writing of data that does not fill a whole byte

<!--
|- Temporary file storage for large data
|-> use `tempfile` crate
-->

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
    assert_eq!(file.read_utf8(size).unwrap(), "Hello, world!\n");
}
```

### Simple file writing

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_w("data.txt").unwrap();
    file.write_utf8_at("Hello, world!\n", 0).unwrap();
    file.close().unwrap(); // optional, but recommended
}
```

### Open a file in read/write mode

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_rw("data.txt").unwrap();
    file.write_utf8_at("Hello, world!\n", 0).unwrap();
    file.rewind().unwrap();
    let size = file.size().unwrap();
    assert_eq!(file.read_utf8(size).unwrap(), "Hello, world!\n");
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
    assert_eq!(rw.read_u8().unwrap(), 31);
}
```

##### Mutable borrowing

```rust
use dh::recommended::*;

fn main() {
    let mut data = vec![0u8; 1];
    let mut rw = dh::data::rw_ref(&mut data);
    rw.write_u8(31).unwrap();
    rw.rewind().unwrap();
    assert_eq!(rw.read_u8().unwrap(), 31);
}
```

#### Alternative: moving

```rust
use dh::recommended::*;

fn main() {
    let data = vec![0u8; 1];
    let mut rw = dh::data::rw(data);
    rw.write_u8(31).unwrap();
    rw.rewind().unwrap();
    assert_eq!(rw.read_u8().unwrap(), 31);

    let data = dh::data::close(rw).unwrap();
    assert_eq!(data, vec![31]);
}
```

### Limit readable space

```rust
use dh::recommended::*;

fn main() {
    let mut file = dh::file::open_r("data.txt").unwrap();
    let mut limited = file.limit(0, 5).unwrap();
    assert_eq!(limited.read_utf8(5).unwrap(), "Hello");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
