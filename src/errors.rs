use csscolorparser::ParseColorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum TimelineError {
    #[error("error parsing palette color `{0}`: {1}")]
    PaletteColorParsing(String, ParseColorError),
    #[error("error parsing canvas style property `color`: {0}")]
    ParsingCanvasColor(ParseColorError),
    #[error("error getting canvas context")]
    GetCanvasContext,
    #[error("error decoding MessagePack data: {0}")]
    MsgPackDecode(rmp_serde::decode::Error),
    #[error("color index is not in palette: {0}")]
    ColorIndexNotInPalette(usize),
    #[error("empty dataset")]
    EmptyDataset,
    #[error("error creating backend")]
    BackendCreation,
}
