# rusttesserast
rusttesserast is a simple library providing the asynchronous methods for tesseract api in Rust.
# Usage
Add this to your Cargo.toml:
```
[dependencies]
```
```
use rusttesserast::tess_lib::TesseractApi;

#[tokio::main]
async fn main() {
    let mut tesseract_base = TesseractApi::new(Some(TesseractApi{dpi: 3, psm:3, ..Default::default()}), Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("eng").as_str())).unwrap();
    let image_array = vec!["/workspaces/rusttesserast/tests/test_img.png", "/workspaces/rusttesserast/tests/test_img.png"];
    let test = tesseract_base.recognize_doc(None, None, image_array, "txt").await;
}

```
# License
Licensed under MIT license.