use rusttesserast::tess_lib::TesseractApi;

fn main() {
    let mut tesseract_base = TesseractApi::new(Some(String::from("/usr/local/share/tessdata").as_str()), Some(String::from("rus").as_str())).unwrap();
    let image_array = vec!["/workspaces/rusttesserast/tests/resipients.png"];
    let test = tesseract_base.recognize_doc(None, None, image_array, "tsv");
    println!("{:?}", test);
}
