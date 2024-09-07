# `dh`

Data handling in Rust, made easy.

## Features

- Read and write files in streams
- Read and write u8 vectors
- Partial read & write access
- Temporary file storage for large data
- No unsafe code
- std::io::Read and std::io::Write implementations

## Installation

```bash
cargo install dh
```

## Usage

### Simple file reading

```rust
use dh;

fn main() {
    let mut file = dh::file::open_r("data.txt").unwrap();
    let size = file.size();
    assert_eq!(file.read_utf8(size), "Hello, world!\n");
}
```

### Simple file writing

```rust
use dh;

fn main() {
    let mut file = dh::file::open_w("data.txt").unwrap();
    file.write_utf8("Hello, world!\n");
    file.close(); // optional, but recommended
}
```

### Open a file in read/write mode

```rust
use dh;

fn main() {
    let mut file = dh::file::open_rw("data.txt").unwrap();
    file.write_utf8("Hello, world!\n");
    file.rewind();
    assert_eq!(file.read_utf8(file.size()), "Hello, world!\n");
}
```

### Read and write u8 vectors

```rust
use dh;

fn main() {
    let mut data = vec![0u8; 1];
    let mut rw = dh::data::open_rw(&mut data);
    rw.write_u8(31);
    rw.rewind();
    assert_eq!(rw.read_u8(), 31);
}
```

or

```rust
use dh;

fn main() {
    let data = vec![0u8; 1];
    let mut rw = dh::data::rw(data);
    rw.write_u8(31);
    rw.rewind();
    assert_eq!(rw.read_u8(), 31);
}
```

### Read and write u8 vectors and temporarily store them in a file

```rust
use dh;

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
use dh;

fn main() {
    let mut file = dh::file::open_r("data.txt").unwrap();
    let mut limited = file.limit(0, 5);
    assert_eq!(limited.read_utf8(5), "Hello");
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
