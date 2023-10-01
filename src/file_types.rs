pub enum OutputFileFormat {
    PDF,
    ALTO,
    HOCR,
    TSV,
    TXT,
    OSD,
}

impl OutputFileFormat {
    pub fn get_type(&self) -> &str {
        match self {
            OutputFileFormat::PDF => "pdf",
            OutputFileFormat::ALTO => "alto",
            OutputFileFormat::HOCR => "hocr",
            OutputFileFormat::TSV => "tsv",
            OutputFileFormat::TXT => "txt",
            OutputFileFormat::OSD => "osd",
        }
    }
}