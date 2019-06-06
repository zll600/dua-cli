use crate::interactive::EntryMarkMap;
use dua::traverse::TreeIndex;
use itertools::Itertools;
use std::borrow::Borrow;
use tui::{
    buffer::Buffer, layout::Rect, style::Color, style::Style, widgets::Block, widgets::Borders,
    widgets::Text,
};
use tui_react::{List, ListProps};

#[derive(Default)]
pub struct MarkPane {
    pub list: List,
}

pub struct MarkPaneProps<'a> {
    pub border_style: Style,
    pub selected: Option<TreeIndex>,
    pub marked: &'a EntryMarkMap,
}

impl MarkPane {
    pub fn render<'a>(
        &mut self,
        props: impl Borrow<MarkPaneProps<'a>>,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let MarkPaneProps {
            border_style,
            selected,
            marked,
        } = props.borrow();

        let block = Block::default()
            .title("Marked Entries")
            .border_style(*border_style)
            .borders(Borders::ALL);
        let entry_in_view = selected.and_then(|idx| {
            marked
                .iter()
                .enumerate()
                .find_position(|(_pos, (&node_index, _))| node_index == idx)
                .map(|(pos, _)| pos)
        });

        let entries = marked.iter().map(|(_, v)| {
            let name = Text::Styled(
                v.path.to_string_lossy(),
                Style {
                    fg: Color::LightRed,
                    ..Style::default()
                },
            );
            vec![name]
        });

        let props = ListProps {
            block: Some(block),
            entry_in_view,
        };
        self.list.render(props, entries, area, buf)
    }
}