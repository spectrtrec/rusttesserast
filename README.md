# Rusttesserast
[![Crates.io](https://img.shields.io/crates/v/rusttesserast.svg)](https://crates.io/crates/rusttesserast)
[![Documentation](https://docs.rs/rusttesserast/badge.svg)](https://docs.rs/rusttesserast)

<p align="center">
  <img src="logo.png" alt="logo"/>
</p>

## Overview
rusttesserast is a simple library providing the asynchronous methods for tesseract api in Rust.
See [documentation](https://docs.rs/rusttesserast) for more.
## Usage
Add this to your Cargo.toml:
```
[dependencies]
rusttesserast = "0.0.1-alpha.4"
```
```rust
use rusttesserast::tess_lib::TesseractApi;

#[tokio::main]
async fn main() {
    let mut tesseract_base = TesseractApi::new(Some(TesseractApi{dpi: 3, psm:3, ..Default::default()}), Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("eng").as_str())).unwrap();
    let image_array = vec!["/workspaces/rusttesserast/tests/test_img.png", "/workspaces/rusttesserast/tests/test_img.png"];
    let test = tesseract_base.recognize_doc(None, None, image_array, "txt").await;
}
```
```rust
use rusttesserast::tess_lib::TesseractApi;

#[tokio::main]
async fn main() {
    let mut tesseract_base = TesseractApi::new(Some(TesseractApi{dpi: 3, psm:3, ..Default::default()}), Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("eng").as_str())).unwrap();
    let image_array = vec!["/workspaces/rusttesserast/tests/test_img.png", "/workspaces/rusttesserast/tests/test_img.png"];
    let test = tesseract_base.recognize_doc("test/save/path", "test_data_file.txt", image_array, "tsv").await;
}
```
## License
Licensed under MIT license.

