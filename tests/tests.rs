use glob::glob;
use rusttesserast::constanst::TesseractDefaultConstants;
use rusttesserast::errors::TesseractError;
use rusttesserast::tess_lib::TesseractApi;
use rusttesserast::utils::get_current_working_dir;
use std::path::{Path, PathBuf};

#[test]
fn test_get_dpi_attr() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("rus").as_str()),
    )?;
    let dps_attr = cube.get_attr("dpi");
    assert_eq!(
        dps_attr.unwrap_left().unwrap(),
        &TesseractDefaultConstants::DEFAULT_DPI
    );
    Ok(())
}
#[test]
fn test_get_psm_attr() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("rus").as_str()),
    )?;
    let psm_attr = cube.get_attr("psm");
    assert_eq!(
        psm_attr.unwrap_right().unwrap(),
        &TesseractDefaultConstants::DEFAULT_PSM
    );
    Ok(())
}

#[test]
fn test_get_oem_attr() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("rus").as_str()),
    )?;
    let oem_attr = cube.get_attr("oem");
    assert_eq!(
        oem_attr.unwrap_right().unwrap(),
        &TesseractDefaultConstants::DEFAULT_OEM
    );
    Ok(())
}

#[test]
fn test_get_timeout_attr() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("rus").as_str()),
    )?;
    let oem_attr = cube.get_attr("timeout");
    assert_eq!(
        oem_attr.unwrap_left().unwrap(),
        &TesseractDefaultConstants::DEFAULT_TIMEOUT
    );
    Ok(())
}

#[test]
fn test_error_attr() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("rus").as_str()),
    )?;
    let error_atr = cube.get_attr("None_exist_attrt");
    assert_eq!(
        error_atr.factor_err(),
        Err(format!(
            "invalid field name to get '{}'",
            "None_exist_attrt"
        ))
    );
    Ok(())
}

#[test]
fn test_new_method_error() -> Result<(), TesseractError> {
    let cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/shares/tessdatas").as_str()),
        Some(String::from("rust").as_str()),
    );
    let expected: Result<(), TesseractError> = Err(TesseractError::TesseractInitError);
    assert_eq!(cube.unwrap_err(), expected.unwrap_err());
    Ok(())
}

#[tokio::test]
async fn test_hocr() -> Result<(), TesseractError> {
    let mut cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )?;
    let result = cube
        .image_to_hocr(String::from("tests/test_img.png").as_str())
        .await;
    assert!(result?.contains(" <div class='ocr_page'"));
    Ok(())
}

#[tokio::test]
async fn test_image_to_string() -> Result<(), TesseractError> {
    let mut cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )?;
    let result = cube
        .image_to_string(String::from("tests/test_img.png").as_str())
        .await;
    assert!(result?.contains("World!"));
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "There was a problem opening the file: NoSuchFileException")]
async fn test_image_to_string_err() -> () {
    let mut cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let result = cube
        .image_to_string(String::from("tests/test_imgk.png").as_str())
        .await
        .unwrap();
    ()
}

#[tokio::test]
async fn test_image_to_tsv() -> Result<(), TesseractError> {
    let mut cube = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )?;
    let result = cube
        .image_to_tsv(String::from("tests/test_img.png").as_str())
        .await?;
    assert_eq!(result, include_str!("data.txt"));
    Ok(())
}

#[tokio::test]
async fn test_recognize_doc() -> Result<(), TesseractError> {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let image_array = vec!["tests/test_img.png"];
    tesseract_base
        .recognize_doc(None, None, image_array, "tsv", None)
        .await?;
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "None existing format tsvs")]
async fn test_recognize_doc_panic() -> () {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let image_array = vec!["tests/test_img.png"];
    tesseract_base
        .recognize_doc(None, None, image_array, "tsvs", None)
        .await;
}

#[test]
fn test_save_doc() -> () {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let doc_vec = vec![String::from("/usr/local/share/tessdata")];
    tesseract_base.save_doc(
        Some(String::from("/usr/local/share/tessdata").as_str()),
        None,
        &doc_vec,
    );
    let files_: Vec<PathBuf> = glob("/usr/local/share/tessdata/data.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    assert!(files_.len() > 0);
    ()
}
#[tokio::test]
async fn test_recognize_doc_save_doc() -> () {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let image_array = vec!["tests/test_img.png"];
    let res_path = get_current_working_dir()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned()
        + "/tests/";
    let res = tesseract_base
        .recognize_doc(
            Some(res_path.as_str()),
            Some(String::from("data_custom.txt").as_str()),
            image_array,
            "txt",
            Some(true),
        )
        .await.unwrap();
    assert_eq!(res.len(), 1);
    println!("{:?}", res);
    let files_: Vec<PathBuf> = glob("/workspaces/rusttesserast/tests/data_custom.txt")
    .unwrap()
    .filter_map(Result::ok)
    .collect();
    assert!(files_.len() > 0);
    ()
}

#[test]
fn test_save_doc_custom_name() -> () {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let doc_vec = vec![String::from("/usr/local/share/tessdata")];
    tesseract_base.save_doc(
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("data_custom.txt").as_str()),
        &doc_vec,
    );
    let files_: Vec<PathBuf> = glob("/usr/local/share/tessdata/data_custom.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    assert!(files_.len() > 0);
    ()
}

#[test]
#[should_panic(expected = "Path /usr/local/share/tessdata_fake doesnt exist. Use another path.")]
fn test_save_doc_panic_path() -> () {
    let mut tesseract_base = TesseractApi::new(
        None,
        Some(String::from("/usr/local/share/tessdata").as_str()),
        Some(String::from("eng").as_str()),
    )
    .unwrap();
    let doc_vec = vec![String::from("/usr/local/share/tessdata")];
    tesseract_base.save_doc(
        Some(String::from("/usr/local/share/tessdata_fake").as_str()),
        Some(String::from("data_custom.txt").as_str()),
        &doc_vec,
    );
    ()
}
