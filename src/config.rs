use std::path::PathBuf;

use async_lsp::lsp_types::{ClientCapabilities, PositionEncodingKind};
use line_index::WideEncoding;

use crate::line_index::PositionEncoding;

pub struct Config {
    pub capabilities: ClientCapabilities,
    pub workspace_folder: PathBuf,
}

impl Config {
    pub fn negotiated_encoding(&self) -> PositionEncoding {
        let client_encodings = match &self.capabilities.general {
            Some(general) => general.position_encodings.as_deref().unwrap_or_default(),
            None => &[],
        };

        for enc in client_encodings {
            if enc == &PositionEncodingKind::UTF8 {
                return PositionEncoding::Utf8;
            } else if enc == &PositionEncodingKind::UTF32 {
                return PositionEncoding::Wide(WideEncoding::Utf32);
            }
            // NB: intentionally prefer just about anything else to utf-16.
        }

        PositionEncoding::Wide(WideEncoding::Utf16)
    }
}
