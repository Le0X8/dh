//! Data handling in Rust, made easy.
//!
//! ---
//!
//! This library provides a set of traits and structs to handle data in Rust,
//! with support for reading, writing, and reading/writing at the same time.
//!
//! Basically, you can read and write files and buffers with the same API.
//!
//! ## Examples
//!
//! ### Reading a file
//!
//! ```rust
//! use dh; // import the base crate
//! use dh::{Readable}; // import the required Readable trait (Writeable and Rw are also available)
//!
//! let mut file = dh::file::open_r("tests/samples/000").unwrap(); // this opens a file exclusively for reading
//!
//! let size = file.size().unwrap(); // get the size of the file
//! let str = file.read_utf8(size).unwrap(); // read the whole file as UTF-8, read_utf8_at can be used to read at a specific position
//!
//! // file will be closed automatically when it goes out of scope, you can also close it manually with file.close()
//!
//! assert_eq!(str, "Hello, world!"); // check if the content is correct
//!
//! println!("{}", str); // print the content
//! ```
//!
//! ### Writing a u8 vector in R/W mode
//!
//! ```rust
//! use dh; // import the base crate
//! use dh::{Readable, Writable}; // import the required traits
//!
//! let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7]; // create a vector
//! let mut rw = dh::data::rw_ref(&mut data); // open the vector in R/W mode, just using dh::data::rw would move the vector
//!
//! rw.write_u8_at(0, 8).unwrap(); // write 8 at the beginning
//! // note how the position stays at 0
//! assert_eq!(rw.read_u64be().unwrap(), 0x0801020304050607); // read a u64 in big-endian
//!
//! dh::data::close_ref(rw).unwrap(); // close the R/W object and get the reference back
//! // you can drop the rw object too if you don't need the reference anymore
//! // you can get the whole vector back with dh::data::close(rw).unwrap() if it was moved
//!
//! assert_eq!(data, vec![8, 1, 2, 3, 4, 5, 6, 7]); // check if the data is correct

/// The whole set of structs and functions to handle u8 vectors.
pub mod data;

/// The whole set of structs and functions to handle files.
pub mod file;

mod read;
mod rw;
mod r#type;
mod write;

pub use data::{ClosableData, ClosableRefData};
pub use r#type::*;
pub use read::*;
pub use rw::*;
pub use write::*;
