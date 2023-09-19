pub enum FileFormat {
    PDF,
    ALTO,
    HOCR,
    TSV,
    TXT,
    OSD,
}

impl FileFormat {
    pub fn get_type(&self) -> &str {
        match self {
            FileFormat::PDF => "pdf",
            FileFormat::ALTO => "alto",
            FileFormat::HOCR => "hocr",
            FileFormat::TSV => "tsv",
            FileFormat::TXT => "txt",
            FileFormat::OSD => "osd",
        }
    }
}