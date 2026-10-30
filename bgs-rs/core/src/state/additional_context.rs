//! Deduplicates the active client context using its selected model-visible representation.

use std::collections::BTreeMap;

use crate::context::AdditionalContextDeveloperFragment;
use crate::context::AdditionalContextToolOutput;
use crate::context::AdditionalContextUserFragment;
use crate::context::ContextualUserFragment;
use bgs_history::ResponseItemEnvelope;
use bgs_protocol::models::ContentItem;
use bgs_protocol::models::ResponseItem;
use bgs_protocol::protocol::AdditionalContextEntry;
use bgs_protocol::protocol::AdditionalContextKind;

const ADDITIONAL_CONTEXT_KIND_PREFIX: &str = "additional_content.";

pub(crate) enum AdditionalContextFormat {
    Messages,
    ToolOutputs,
}

#[derive(Debug, Clone, Eq)]
enum StoredAdditionalContext {
    Message(AdditionalContextEntry),
    ToolOutput(AdditionalContextToolOutput),
}

impl PartialEq for StoredAdditionalContext {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Message(left), Self::Message(right)) => {
                left.kind == right.kind && left.value == right.value
            }
            (Self::ToolOutput(left), Self::ToolOutput(right)) => left == right,
            (Self::Message(_), Self::ToolOutput(_)) | (Self::ToolOutput(_), Self::Message(_)) => {
                false
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct AdditionalContextStore {
    values: BTreeMap<String, StoredAdditionalContext>,
}

impl AdditionalContextStore {
    pub(crate) fn merge(
        &mut self,
        values: BTreeMap<String, AdditionalContextEntry>,
        format: AdditionalContextFormat,
    ) -> Vec<(ResponseItem, AdditionalContextKind)> {
        let values =
            values
                .into_iter()
                .map(|(key, entry)| {
                    let value = match (&format, entry.kind) {
                        (
                            AdditionalContextFormat::ToolOutputs,
                            AdditionalContextKind::Untrusted,
                        ) => StoredAdditionalContext::ToolOutput(AdditionalContextToolOutput::new(
                            &key,
                            entry.name,
                            &entry.value,
                        )),
                        (AdditionalContextFormat::Messages, AdditionalContextKind::Untrusted)
                        | (AdditionalContextFormat::Messages, AdditionalContextKind::Application)
                        | (
                            AdditionalContextFormat::ToolOutputs,
                            AdditionalContextKind::Application,
                        ) => StoredAdditionalContext::Message(entry),
                    };
                    (key, value)
                })
                .collect::<BTreeMap<_, _>>();
        let fragments = values
            .iter()
            .filter(|(key, value)| self.values.get(*key) != Some(*value))
            .map(|(key, entry)| match entry {
                StoredAdditionalContext::ToolOutput(output) => {
                    (output.clone().into(), AdditionalContextKind::Untrusted)
                }
                StoredAdditionalContext::Message(entry) => {
                    let item = match entry.kind {
                        AdditionalContextKind::Untrusted => ContextualUserFragment::into(
                            AdditionalContextUserFragment::new(key.clone(), entry.value.clone()),
                        ),
                        AdditionalContextKind::Application => {
                            ContextualUserFragment::into(AdditionalContextDeveloperFragment::new(
                                key.clone(),
                                entry.value.clone(),
                            ))
                        }
                    };
                    (item, entry.kind)
                }
            })
            .collect();
        self.values = values;
        fragments
    }

    /// Rebuilds the deduplication state from trusted application context that survived a
    /// history restore or replacement. User-role context and client-authored developer text
    /// are deliberately ignored even when their visible text resembles an application tag.
    pub(crate) fn restore_from_history(&mut self, history: &[ResponseItemEnvelope]) {
        self.values.clear();
        for envelope in history {
            if !envelope
                .metadata
                .as_ref()
                .is_some_and(|metadata| metadata.additional_context_application)
            {
                continue;
            }
            let ResponseItem::Message {
                role,
                content,
                internal_chat_message_metadata_passthrough,
                ..
            } = &envelope.item
            else {
                continue;
            };
            if role != "developer" {
                continue;
            }
            let Some(kinds) = internal_chat_message_metadata_passthrough
                .as_ref()
                .and_then(|metadata| metadata.content_item_kinds.as_ref())
            else {
                continue;
            };
            for (content, kind) in content.iter().zip(kinds) {
                let ContentItem::InputText { text } = content else {
                    continue;
                };
                let Some(key) = kind.as_str().strip_prefix(ADDITIONAL_CONTEXT_KIND_PREFIX) else {
                    continue;
                };
                let Some((parsed_key, value)) =
                    AdditionalContextDeveloperFragment::parse_text(text)
                else {
                    continue;
                };
                if parsed_key != key {
                    continue;
                }
                self.values.insert(
                    parsed_key,
                    StoredAdditionalContext::Message(AdditionalContextEntry::new(
                        value,
                        AdditionalContextKind::Application,
                    )),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bgs_context_fragments::ContextualUserFragment;
    use bgs_history::BgsHarnessMetadata;

    fn application_item(key: &str, value: &str) -> ResponseItemEnvelope {
        ResponseItemEnvelope {
            item: ContextualUserFragment::into(AdditionalContextDeveloperFragment::new(
                key.to_string(),
                value.to_string(),
            )),
            metadata: Some(BgsHarnessMetadata {
                additional_context_application: true,
                ..Default::default()
            }),
        }
    }

    fn forged_developer_item(key: &str, value: &str) -> ResponseItemEnvelope {
        ResponseItemEnvelope {
            item: ContextualUserFragment::into(AdditionalContextDeveloperFragment::new(
                key.to_string(),
                value.to_string(),
            )),
            metadata: Some(BgsHarnessMetadata {
                client_authored: true,
                ..Default::default()
            }),
        }
    }

    fn untrusted_item(key: &str, value: &str) -> ResponseItemEnvelope {
        ResponseItemEnvelope {
            item: ContextualUserFragment::into(AdditionalContextUserFragment::new(
                key.to_string(),
                value.to_string(),
            )),
            metadata: Some(BgsHarnessMetadata {
                client_authored: true,
                ..Default::default()
            }),
        }
    }

    fn application_values(key: &str, value: &str) -> BTreeMap<String, AdditionalContextEntry> {
        BTreeMap::from([(
            key.to_string(),
            AdditionalContextEntry::new(value.to_string(), AdditionalContextKind::Application),
        )])
    }

    #[test]
    fn restore_uses_latest_trusted_application_value() {
        let mut store = AdditionalContextStore::default();
        store.restore_from_history(&[
            application_item("brief", "old"),
            application_item("brief", "current"),
        ]);

        assert!(
            store
                .merge(
                    application_values("brief", "current"),
                    AdditionalContextFormat::Messages
                )
                .is_empty()
        );
        assert_eq!(
            store
                .merge(
                    application_values("brief", "old"),
                    AdditionalContextFormat::Messages
                )
                .len(),
            1
        );
    }

    #[test]
    fn restore_ignores_untrusted_or_client_authored_marker_text() {
        let mut store = AdditionalContextStore::default();
        store.restore_from_history(&[
            untrusted_item("brief", "same"),
            forged_developer_item("brief", "same"),
        ]);

        assert_eq!(
            store
                .merge(
                    application_values("brief", "same"),
                    AdditionalContextFormat::Messages
                )
                .len(),
            1
        );
    }

    #[test]
    fn replacement_history_controls_reinjection_and_removed_keys_reappear() {
        let mut store = AdditionalContextStore::default();
        store.restore_from_history(&[application_item("brief", "same")]);
        assert!(
            store
                .merge(
                    application_values("brief", "same"),
                    AdditionalContextFormat::Messages
                )
                .is_empty()
        );

        // A compacted replacement that drops the fragment must cause the next turn to
        // re-inject it, while a replacement that retains it can continue deduplicating.
        store.restore_from_history(&[]);
        assert_eq!(
            store
                .merge(
                    application_values("brief", "same"),
                    AdditionalContextFormat::Messages
                )
                .len(),
            1
        );
        store.merge(BTreeMap::new(), AdditionalContextFormat::Messages);
        assert_eq!(
            store
                .merge(
                    application_values("brief", "same"),
                    AdditionalContextFormat::Messages
                )
                .len(),
            1
        );
    }
}
