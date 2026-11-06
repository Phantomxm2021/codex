//! Bounded recovery guidance for a response blocked by an unspecified content filter.

use super::ContextualUserFragment;
use bgs_protocol::models::ContentItem;
use bgs_protocol::models::ContentItemKind;
use bgs_protocol::models::input_text;

pub(crate) struct ContentFilterGuidance {
    pub(crate) text: String,
}

impl ContextualUserFragment for ContentFilterGuidance {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("generic.content_filter_guidance".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<content_filter_guidance>", "</content_filter_guidance>")
    }

    fn body(&self) -> ContentItem {
        input_text(format!("\n{}\n", self.text))
    }
}
