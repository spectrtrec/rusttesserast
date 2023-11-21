//! Simple library providing asynchronous methods for tesseract api.
//!
//! Main function `recognize_doc` allows to recognize images and save data to a file. This function takes four parameters:
//! 1. save_path: Option<&str> - path for saving a doc. If path is None, then file will be saved to a project dir.
//! 2. doc_name: Option<&str> - name of a doc where recognized information will be saved. Default value - data.txt
//! 3. image_array: Vec<&str> - vector which contains a paths to available images.
//! 4. output_type: &str - a str which contains output type value. So far, only 4 types available (txt, tsv, hocr)
//!
//! # Examples
//!
//! ```rust, no_run
//! use rusttesserast::tess_lib::TesseractApi;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut tesseract_base = TesseractApi::new(Some(TesseractApi{dpi: 3, psm:3, ..Default::default()}), Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("eng").as_str())).unwrap();
//!     let image_array = vec!["/workspaces/rusttesserast/tests/test_img.png", "/workspaces/rusttesserast/tests/test_img.png"];
//!     let test = tesseract_base.recognize_doc(None, None, image_array, "txt").await;
//! }
//! ```
//!
//! ```rust, no_run
//! use rusttesserast::tess_lib::TesseractApi;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut tesseract_base = TesseractApi::new(Some(TesseractApi{dpi: 3, psm:3, ..Default::default()}), Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("eng").as_str())).unwrap();
//!     let image_array = vec!["/workspaces/rusttesserast/tests/test_img.png", "/workspaces/rusttesserast/tests/test_img.png"];
//!     let test = tesseract_base.recognize_doc("test/save/path", "test_data_file.txt", image_array, "tsv").await;
//! }
//!
#![doc(

    html_root_url = "https://docs.rs/rusttesserast/0.0.1-alpha.4"
)]
pub mod constanst;
pub mod file_types;
pub mod errors;
pub mod tess_lib;
pub mod utils;