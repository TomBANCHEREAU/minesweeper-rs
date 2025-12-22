use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileStatus {
    Unknown,
    Flagged,
    Hint(u8),
}

impl Form<&TileStatus> for &str {
    fn into(self) -> &str {
        todo!()
    }
}

#[component]
pub fn tile(status: &TileStatus) -> Element {
    rsx! {
      button {
            }
    }
}
