use crate::constanst::TesseractDefaultConstants;
use crate::errors::TesseractError;
use crate::file_types::OutputFileFormat;
use crate::utils::get_current_working_dir;
use std::ffi::CString;
use std::fs::{metadata, File};
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::process::Command;
use tesseract_plumbing as pl;

#[derive(Default)]
pub struct TesseractApi {
    dpi: i32,
    psm: u32,
    oem: u32,
    timeout: i32,
    tess_pl: pl::TessBaseApi,
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

    pub fn new(datapath: Option<&str>, lang: Option<&str>) -> Result<TesseractApi, TesseractError> {
        let mut tesseract = TesseractApi::default();
        tesseract.tess_pl.set_source_resolution(tesseract.dpi);
        tesseract.tess_pl.set_page_seg_mode(tesseract.psm);
        let datapath = match datapath {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None,
        };
        let lang = match lang {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None,
        };
        tesseract
            .tess_pl
            .init_4(datapath.as_deref(), lang.as_deref(), tesseract.oem)
            .ok();
        Ok(tesseract)
    }

    fn image_to_string(&mut self, filename: &str) -> Result<String, TesseractError> {
        match pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()) {
            Ok(pix) => self.tess_pl.set_image_2(&pix),
            Err(PixReadError) => return Err(TesseractError::TesseractInitError),
        };

        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    fn image_to_hocr(&mut self, filename: &str) -> Result<String, TesseractError> {
        self.tess_pl.set_image_2(
            &pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()).unwrap(),
        );
        Ok(self
            .tess_pl
            .get_hocr_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    fn image_to_tsv(&mut self, filename: &str) -> Result<String, TesseractError> {
        self.tess_pl.set_image_2(
            &pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()).unwrap(),
        );
        Ok(self
            .tess_pl
            .get_tsv_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    fn get_text(&mut self) -> Result<String, TesseractError> {
        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    fn iter_through_img(
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

    fn save_doc(&mut self, path: Option<&str>, file_name: Option<&str>, doc_vec: Vec<String>) {
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
            "pdf" => OutputFileFormat::PDF,
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
        
        self.save_doc(
            save_path,
            doc_name,
            doc,
        );
        Ok(())
    }
}
