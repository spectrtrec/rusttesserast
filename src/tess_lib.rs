use tesseract_plumbing as pl;
use std::ffi::CString;
use crate::constanst::TesseractDefaultConstants;
use crate::errors::TesseractError;

#[derive(Default)]
pub struct TesseractApi{
    dpi:i32,
    psm:u32,
    oem:u32,
    timeout:i32,
    tess_pl:pl::TessBaseApi,
}

impl TesseractApi{
    fn default () -> Self {
        return TesseractApi{
            dpi:TesseractDefaultConstants::DEFAULT_DPI,
            psm:TesseractDefaultConstants::DEFAULT_PSM,
            oem:TesseractDefaultConstants::DEFAULT_OEM,
            timeout:TesseractDefaultConstants::DEFAULT_TIMEOUT,
            tess_pl:pl::TessBaseApi::create()
        }
     }
    
    
    pub fn new(datapath:Option<&str>, lang:Option<&str>) -> Result<TesseractApi, TesseractError>{
        let mut tesseract = TesseractApi::default();
        tesseract.tess_pl.set_source_resolution(tesseract.dpi);
        tesseract.tess_pl.set_page_seg_mode(tesseract.psm);
        let datapath = match  datapath {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None
        };
        let lang = match lang {
            Some(i) => Some(CString::new(i).unwrap()),
            None => None
        };
        tesseract.tess_pl.init_4(datapath.as_deref(), lang.as_deref(), tesseract.oem).ok();
        Ok(tesseract)
    }
    
    pub fn image_to_string(mut self, filename: &str) -> Result<String, TesseractError> {
        
        match pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()) {
            Ok(pix ) => self.tess_pl.set_image_2(&pix),
            Err(PixReadError) => return Err(TesseractError::TesseractInitError)
        };
        
        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned()
        )
    }

    pub fn image_to_hocr(mut self, filename: &str) -> Result<String, TesseractError> {
        self.tess_pl.set_image_2(&pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()).unwrap());
        Ok(self
            .tess_pl
            .get_hocr_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned()
        )
    }
    pub fn image_to_tsv(mut self, filename: &str) -> Result<String, TesseractError> {
        self.tess_pl.set_image_2(&pl::leptonica_plumbing::Pix::read(&CString::new(filename).unwrap()).unwrap());
        Ok(self
            .tess_pl
            .get_tsv_text(0)
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned()
        )
    }
    
    pub fn get_text(&mut self) -> Result<String, TesseractError> {
        Ok(self
            .tess_pl
            .get_utf8_text()
            .unwrap()
            .as_ref()
            .to_string_lossy()
            .into_owned()
        )
    }
 }