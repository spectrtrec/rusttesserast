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

use crate::constanst::TesseractDefaultConstants;
use crate::errors::TesseractError;
use crate::file_types::OutputFileFormat;
use crate::utils::get_current_working_dir;
use derivative::Derivative;
use either::*;
use futures::prelude::*;
use futures::stream::FuturesOrdered;
use pl::TessBaseApiInitError;
use std::ffi::CString;
use std::fs::{metadata, File};
use std::io::Write;
use std::path::Path;
use tesseract_plumbing as pl;

#[derive(Derivative)]
#[derivative(Default, Debug)]
/// TesseractApi is a base pub structure for entire project.
pub struct TesseractApi {
    #[derivative(Default(value = "300"))]
    /// dpi (or Dot Per Inch) - is a measure of spatial printing, video, or image scanner dot density.
    /// Tesseract works best on images with a Dot Per Inch (DPI) of at least 300 dpi.
    /// All available options are described here - `https://github.com/tesseract-ocr/tesseract/blob/main/doc/tesseract.1.asc`.
    /// Default value - 300
    pub dpi: i32,
    #[derivative(Default(value = "4"))]
    /// psm - is a Tesseract Page Segmentation Modes.
    /// All available options are described here - `https://github.com/tesseract-ocr/tesseract/blob/main/doc/tesseract.1.asc`.
    /// Default value - 4
    pub psm: u32,
    #[derivative(Default(value = "3"))]
    /// oem - is a Tesseract Engine modes.
    /// All available options are described here - `https://github.com/tesseract-ocr/tesseract/blob/main/doc/tesseract.1.asc`.
    /// Default value - 3
    pub oem: u32,
    #[derivative(Default(value = "30"))]
    pub timeout: i32,
    /// tesseract_plumbing base api
    pub tess_pl: pl::TessBaseApi,
}

/// TesseractApi is a base Implementation for pub struct TesseractApi.
impl TesseractApi {
    fn default() -> Self {
        return TesseractApi {
            dpi: TesseractDefaultConstants::DEFAULT_DPI,
            psm: TesseractDefaultConstants::DEFAULT_PSM,
            oem: TesseractDefaultConstants::DEFAULT_OEM,
            timeout: TesseractDefaultConstants::DEFAULT_TIMEOUT,
            tess_pl: pl::TessBaseApi::create(),
        };
    }

    pub fn get_attr(
        &self,
        field_string: &str,
    ) -> Either<Result<&i32, String>, Result<&u32, String>> {
        match field_string {
            "dpi" => Left(Ok(&self.dpi)),
            "psm" => Right(Ok(&self.psm)),
            "oem" => Right(Ok(&self.oem)),
            "timeout" => Left(Ok(&self.timeout)),
            _ => Right(Err(format!("invalid field name to get '{}'", field_string))),
        }
    }

    pub fn new(
        tesseract: Option<TesseractApi>,
        datapath: Option<&str>,
        lang: Option<&str>,
    ) -> Result<TesseractApi, TesseractError> {
        // Create instance of a tesseract api.
        // tesseract - TesseractApi object.
        // datapath - path to tesseract exec file.
        // lang - tesseract languages. Tesseract support more then a 100 languages.
        let mut tess = match tesseract {
            Some(tesseract) => tesseract,
            None => TesseractApi::default(),
        };

        tess.tess_pl.set_source_resolution(tess.dpi);
        tess.tess_pl.set_page_seg_mode(tess.psm);

        let datapath = match datapath {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None,
        };
        let lang = match lang {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None,
        };
        match tess
            .tess_pl
            .init_4(datapath.as_deref(), lang.as_deref(), tess.oem)
        {
            Ok(()) => Ok(tess),
            Err(TessBaseApiInitError {}) => Err(TesseractError::TesseractInitError),
        }
    }

    pub fn set_image(&mut self, filename: &str) -> Result<(), TesseractError> {
        match pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()) {
            Ok(pix) => self.tess_pl.set_image_2(&pix),
            Err(_) => panic!(
                "There was a problem opening the file: {:?}",
                TesseractError::NoSuchFileException
            ),
        };
        Ok(())
    }

    pub fn image_to_string(
        &mut self,
        filename: &str,
    ) -> futures::future::Ready<Result<String, TesseractError>> {
        self.set_image(filename);
        future::ok(
            self.tess_pl
                .get_utf8_text()
                .unwrap()
                .as_ref()
                .to_string_lossy()
                .into_owned(),
        )
    }

    pub fn image_to_hocr(
        &mut self,
        filename: &str,
    ) -> futures::future::Ready<Result<String, TesseractError>> {
        self.set_image(filename);
        future::ok(
            self.tess_pl
                .get_hocr_text(0)
                .unwrap()
                .as_ref()
                .to_string_lossy()
                .into_owned(),
        )
    }

    pub fn image_to_tsv(
        &mut self,
        filename: &str,
    ) -> futures::future::Ready<Result<String, TesseractError>> {
        self.set_image(filename);
        future::ok(
            self.tess_pl
                .get_tsv_text(0)
                .unwrap()
                .as_ref()
                .to_string_lossy()
                .into_owned(),
        )
    }

    #[allow(dead_code)]
    async fn get_text(&mut self) -> futures::future::Ready<Result<String, TesseractError>> {
        future::ok(
            self.tess_pl
                .get_utf8_text()
                .unwrap()
                .as_ref()
                .to_string_lossy()
                .into_owned(),
        )
    }

    pub async fn iter_through_img(
        &mut self,
        api_ogject: fn(
            &mut TesseractApi,
            &str,
        ) -> futures::future::Ready<Result<String, TesseractError>>,
        image_array: Vec<&str>,
    ) -> Vec<Result<String, TesseractError>> {
        let mut rec_vec: FuturesOrdered<future::Ready<Result<String, TesseractError>>> =
            FuturesOrdered::new();
        for image in image_array.iter() {
            rec_vec.push_back(api_ogject(self, image));
        }
        rec_vec.collect().await
    }

    pub fn save_doc(&mut self, path: Option<&str>, file_name: Option<&str>, doc_vec: &Vec<String>) {
        // This function set path and file_name, join texts with \n sep and save doc.
        // path - optional path for saving a doc. If path is None, then file will be saved to a project dir.
        // file_name - optional name of a doc. Default value - data.txt.
        // doc_vec - vector with recognized documents.
        let binding = get_current_working_dir();

        let path = match path {
            Some(path) => path,
            None => binding.as_os_str().to_str().unwrap(),
        };

        let defaul_filename = String::from("data.txt");

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => &defaul_filename,
        };

        if !Path::new(path).exists() {
            format!("Path {path} doesnt exist.");
            panic!("Path {} doesnt exist. Use another path.", path)
        }

        let mut data_file =
            File::create(path.to_owned() + "/" + file_name).expect("creation failed");
        data_file
            .write(doc_vec.join("\n").as_bytes())
            .expect("Unable to write file");
    }
    pub async fn recognize_doc(
        &mut self,
        save_path: Option<&str>,
        doc_name: Option<&str>,
        image_array: Vec<&str>,
        output_type: &str,
        save_doc: Option<bool>,
    ) -> Result<Vec<String>, TesseractError> {
        // This is a base function which recognize and save doc.
        // save_path - optional path for saving a doc. If path is None, then file will be saved to a project dir.
        // doc_name - optional name of a doc. Default value - data.txt
        // image_array - vector which contains a paths to available images.
        // output_type - a str which contains output type value. So far, only 4 types available
        // (txt, tsv, hocr)
        let output_type = match output_type {
            "txt" => OutputFileFormat::TXT,
            "tsv" => OutputFileFormat::TSV,
            "hocr" => OutputFileFormat::HOCR,
            _ => panic!("None existing format {}", output_type),
        };
        let doc = match output_type {
            OutputFileFormat::TXT => {
                let api_ogject: fn(
                    &mut TesseractApi,
                    &str,
                )
                    -> future::Ready<Result<String, TesseractError>> =
                    TesseractApi::image_to_string;
                self.iter_through_img(api_ogject, image_array).await
            }
            OutputFileFormat::TSV => {
                let api_ogject: fn(
                    &mut TesseractApi,
                    &str,
                )
                    -> future::Ready<Result<String, TesseractError>> = TesseractApi::image_to_tsv;
                self.iter_through_img(api_ogject, image_array).await
            }
            OutputFileFormat::HOCR => {
                let api_ogject: fn(
                    &mut TesseractApi,
                    &str,
                )
                    -> future::Ready<Result<String, TesseractError>> = TesseractApi::image_to_hocr;
                self.iter_through_img(api_ogject, image_array).await
            }
            _ => panic!("None existing format"),
        };

        let filtered_doc: Vec<String> = doc.into_iter().filter_map(|s| s.ok()).collect();
        if save_doc.unwrap_or(false) {
            self.save_doc(save_path, doc_name, &filtered_doc);
        }
        Ok(filtered_doc)
    }
}
