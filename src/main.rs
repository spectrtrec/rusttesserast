use rusttesserast::tess_lib::TesseractApi;

fn main() {
    let tesseract_base = TesseractApi::new(Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("rus").as_str())).unwrap();
    let text = tesseract_base.image_to_string(&String::from("/workspaces/rusttesserast/tests/resipients.png").as_str());
    println!("{:?}", text);
}
