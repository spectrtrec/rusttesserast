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
pub struct TesseractApi {
    #[derivative(Default(value = "300"))]
    pub dpi: i32,
    #[derivative(Default(value = "4"))]
    pub psm: u32,
    #[derivative(Default(value = "3"))]
    pub oem: u32,
    #[derivative(Default(value = "30"))]
    pub timeout: i32,
    pub tess_pl: pl::TessBaseApi,
}

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

    pub fn save_doc(&mut self, path: Option<&str>, file_name: Option<&str>, doc_vec: Vec<String>) {
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
            panic!("Path {path} doesnt exist. Use another path.")
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
    ) -> Result<(), TesseractError> {
        let output_type = match output_type {
            "txt" => OutputFileFormat::TXT,
            "tsv" => OutputFileFormat::TSV,
            "HOCR" => OutputFileFormat::HOCR,
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
        self.save_doc(
            save_path,
            doc_name,
            doc.into_iter().filter_map(|s| s.ok()).collect(),
        );
        Ok(())
    }
}
