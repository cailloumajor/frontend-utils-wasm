use hex_color::ParseHexColorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum DrawInternalError {
    #[error("empty dataset")]
    EmptyDataset,
    #[error("canvas not found")]
    CanvasNotFound,
    #[error("{source} (on value `{value}`)")]
    HexColorParse {
        source: ParseHexColorError,
        value: String,
    },
}
