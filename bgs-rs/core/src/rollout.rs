use crate::config::Config;
pub use bgs_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use bgs_rollout::Cursor;
pub use bgs_rollout::INTERACTIVE_SESSION_SOURCES;
pub use bgs_rollout::RolloutRecorder;
pub use bgs_rollout::RolloutRecorderParams;
pub use bgs_rollout::SESSIONS_SUBDIR;
pub use bgs_rollout::SessionMeta;
pub use bgs_rollout::SortDirection;
pub use bgs_rollout::ThreadItem;
pub use bgs_rollout::ThreadSortKey;
pub use bgs_rollout::ThreadsPage;
pub use bgs_rollout::append_thread_name;
pub use bgs_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use bgs_rollout::find_conversation_path_by_id_str;
pub use bgs_rollout::find_thread_meta_by_name_str;
pub use bgs_rollout::find_thread_name_by_id;
pub use bgs_rollout::find_thread_names_by_ids;
pub use bgs_rollout::find_thread_path_by_id_str;
pub use bgs_rollout::parse_cursor;
pub use bgs_rollout::read_head_for_summary;
pub use bgs_rollout::read_session_meta_line;
pub use bgs_rollout::rollout_date_parts;

impl bgs_rollout::RolloutConfigView for Config {
    fn bgs_home(&self) -> &std::path::Path {
        self.bgs_home.as_path()
    }

    fn sqlite_config(&self) -> &bgs_state::SqliteConfig {
        self.sqlite_config()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub(crate) mod list {
    pub use bgs_rollout::find_thread_path_by_id_str;
}

#[cfg(test)]
pub(crate) mod recorder {
    pub use bgs_rollout::RolloutRecorder;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
