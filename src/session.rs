use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::{self, ModeColor},
    view::{Block, View, Separator},
};

pub struct Session;

impl Session {
    pub fn render(name: Option<&str>, mode: InputMode, palette: Palette) -> View {
        let mut blocks = vec![];
        let mut total_len = 0;

        let ModeColor { fg, bg } = ModeColor::new(mode, palette);

        // name
        if let Some(name) = name {
            let text = format!(" {} ", name.to_uppercase());
            let len = text.width();
            let body = style!(fg, bg).bold().paint(text);

            total_len += len;
            blocks.push(Block {
                body: body.to_string(),
                len,
                tab_index: None,
            })
        }

        // separator
        {
            let separator = Separator::render("", &bg, &color::LIGHTER_GRAY);

            total_len += separator.len;
            blocks.push(separator);
        }

        // mode
        {
            let text = {
                let sym = match mode {
                    InputMode::Locked => "".to_string(),
                    InputMode::Normal => "".to_string(),
                    InputMode::Pane => "".to_string(),
                    _ => format!("{:?}", mode).to_uppercase(),
                };

                format!(" {} ", sym)
            };
            let len = text.width();
            let body = style!(palette.white, color::LIGHTER_GRAY).paint(text);

            total_len += len;
            blocks.push(Block {
                body: body.to_string(),
                len,
                tab_index: None,
            })
        }

        // separator
        {
            let separator = Separator::render("", &color::LIGHTER_GRAY, &palette.bg);

            total_len += separator.len;
            blocks.push(separator);
        }

        View {
            blocks,
            len: total_len,
        }
    }
}
