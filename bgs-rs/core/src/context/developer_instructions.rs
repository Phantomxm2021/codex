use super::ContextualUserFragment;
use bgs_protocol::models::ContentItem;
use bgs_protocol::models::ContentItemKind;
use bgs_protocol::models::ContentItemMetadata;
use bgs_protocol::models::input_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeveloperInstructions {
    instructions: String,
}

impl DeveloperInstructions {
    pub(crate) fn new(instructions: impl Into<String>) -> Self {
        Self {
            instructions: instructions.into(),
        }
    }
}

impl ContextualUserFragment for DeveloperInstructions {
    fn content_metadata(&self) -> ContentItemMetadata {
        ContentItemMetadata::developer_instructions(/*from_additional_requirements*/ false)
    }

    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("generic.developer_instructions".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> ContentItem {
        input_text(self.instructions.clone())
    }
}
