use crate::constanst::TesseractDefaultConstants;
use crate::errors::TesseractError;
use crate::file_types::OutputFileFormat;
use crate::utils::get_current_working_dir;
use derivative::Derivative;
use either::*;
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
            Err(_) => return Err(TesseractError::NoSuchFileException),
        };
        Ok(())
    }

    pub fn image_to_string(&mut self, filename: &str) -> Result<String, TesseractError> {
        self.set_image(filename)?;
        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    pub fn image_to_hocr(&mut self, filename: &str) -> Result<String, TesseractError> {
        self.set_image(filename)?;
        Ok(self
            .tess_pl
            .get_hocr_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    pub fn image_to_tsv(&mut self, filename: &str) -> Result<String, TesseractError> {
        self.set_image(filename)?;
        Ok(self
            .tess_pl
            .get_tsv_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    #[allow(dead_code)]
    fn get_text(&mut self) -> Result<String, TesseractError> {
        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    pub fn iter_through_img(
        &mut self,
        api_ogject: fn(&mut TesseractApi, &str) -> Result<String, TesseractError>,
        image_array: Vec<&str>,
    ) -> Vec<String> {
        let mut rec_vec: Vec<String> = Vec::new();
        for image in image_array.iter() {
            rec_vec.push(api_ogject(self, image).unwrap());
        }
        rec_vec
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

    pub fn recognize_doc(
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
                self.iter_through_img(TesseractApi::image_to_string, image_array)
            }
            OutputFileFormat::TSV => self.iter_through_img(TesseractApi::image_to_tsv, image_array),
            OutputFileFormat::HOCR => {
                self.iter_through_img(TesseractApi::image_to_hocr, image_array)
            }
            _ => panic!("None existing format"),
        };

        self.save_doc(save_path, doc_name, doc);
        Ok(())
    }
}
