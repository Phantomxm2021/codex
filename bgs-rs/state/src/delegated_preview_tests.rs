//! Verify delegated preview decoding and tool identity filtering.

use super::delegated_output_preview;
use bgs_protocol::items::FunctionCallOutputItem;
use bgs_protocol::models::FunctionCallOutputBody;
use pretty_assertions::assert_eq;

#[test]
fn delegated_previews_unwrap_only_recognized_inputs() {
    let wrapped = "<bgs_delegation>\n  <source_thread_id>source</source_thread_id>\n  <input>Check &lt;main&gt; &amp; &amp;lt;literal&amp;gt;</input>\n</bgs_delegation>";
    for (namespace, name, text, expected) in [
        (
            Some("bgs_app"),
            "create_thread",
            wrapped,
            Some("Check <main> & &lt;literal&gt;"),
        ),
        (
            Some("bgs_tui"),
            "send_message_to_thread",
            wrapped,
            Some("Check <main> & &lt;literal&gt;"),
        ),
        (
            Some("bgs_app"),
            "create_thread",
            "plain &lt;text&gt;",
            Some("plain &lt;text&gt;"),
        ),
        (Some("bgs_tui"), "create_thread", "  ", None),
        (Some("other"), "create_thread", wrapped, None),
        (None, "create_thread", wrapped, None),
        (Some("bgs_app"), "shell", wrapped, None),
        (
            Some("bgs_app"),
            "create_thread",
            "<bgs_delegation>incomplete",
            Some("<bgs_delegation>incomplete"),
        ),
    ] {
        let output = FunctionCallOutputItem {
            id: "output".to_string(),
            name: name.to_string(),
            namespace: namespace.map(str::to_string),
            output: FunctionCallOutputBody::Text(text.to_string()),
        };
        assert_eq!(delegated_output_preview(&output).as_deref(), expected);
    }
}
