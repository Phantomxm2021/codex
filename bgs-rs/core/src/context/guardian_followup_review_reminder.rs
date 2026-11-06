use super::ContextualUserFragment;
use bgs_protocol::models::ContentItem;
use bgs_protocol::models::ContentItemKind;
use bgs_protocol::models::input_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GuardianFollowupReviewReminder;

impl ContextualUserFragment for GuardianFollowupReviewReminder {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("guardian.followup_review_reminder".to_string())
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
        input_text(bgs_guardian_reviewer::GUARDIAN_FOLLOWUP_REVIEW_REMINDER.to_string())
    }
}
