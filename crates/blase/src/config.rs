use async_lsp::lsp_types::{
    ClientCapabilities, PositionEncodingKind,
};
use camino::Utf8PathBuf;
use line_index::WideEncoding;

use crate::line_index::PositionEncoding;

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub name: String,
}

pub struct Config {
    pub capabilities: ClientCapabilities,
    pub workspace_folder: Utf8PathBuf,
    pub client_info: Option<ClientInfo>,
}

impl Config {
    pub fn client_is_neovim(&self) -> bool {
        self.client_info
            .as_ref()
            .map(|it| it.name == "Neovim")
            .unwrap_or_default()
    }

    pub fn workspace_folder(&self) -> Utf8PathBuf {
        self.workspace_folder.clone()
    }

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
