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
    <b>Binary data handling in Rust, made easy.</b>
</p>

## Features

- Read and write files in streams
- Read and write u8 vectors
- std::io::Read and std::io::Write implementations for `ReadVal` and `WriteVal` (happens automatically as they extend these traits)
- Copying data from `ReadVal` to `Write` (chunked and all at once if you want)
- Floating point number support

### Planned features

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
