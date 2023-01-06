mod multipass_cmd;
mod tesseract_cmd;

pub use multipass_cmd::*;
pub use tesseract_cmd::*;

#[derive(Debug)]
pub enum WarpCmd {
    Tesseract(TesseractCmd),
    MultiPass(MultiPassCmd),
}
