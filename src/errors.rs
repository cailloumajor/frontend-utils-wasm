use colorsys::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum TimelineError {
    #[error("error parsing canvas style property `color`: {0}")]
    ParsingCanvasColor(ParseError),
    #[error("error getting canvas context")]
    GetCanvasContext,
    #[error("bad response status: {0}")]
    ResponseStatus(u16),
    #[error("empty dataset")]
    EmptyDataset,
    #[error("error creating backend")]
    BackendCreation,
}
