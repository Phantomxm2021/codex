use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use bgs_analytics::AnalyticsEventsClient;
use bgs_analytics::AnalyticsJsonRpcError;
use bgs_analytics::InputError;
use bgs_analytics::TurnSteerRequestError;
use bgs_app_server_protocol::Account;
use bgs_app_server_protocol::AccountLoginCompletedNotification;
use bgs_app_server_protocol::AccountTokenUsageDailyBucket;
use bgs_app_server_protocol::AccountTokenUsageSummary;
use bgs_app_server_protocol::AccountUpdatedNotification;
use bgs_app_server_protocol::AddCreditsNudgeCreditType;
use bgs_app_server_protocol::AddCreditsNudgeEmailStatus;
use bgs_app_server_protocol::AdditionalContextEntry;
use bgs_app_server_protocol::AdditionalContextKind;
use bgs_app_server_protocol::AppListUpdatedNotification;
use bgs_app_server_protocol::AppSummary;
use bgs_app_server_protocol::AppTemplateSummary;
use bgs_app_server_protocol::AppTemplateUnavailableReason;
use bgs_app_server_protocol::AppsInstalledParams;
use bgs_app_server_protocol::AppsInstalledResponse;
use bgs_app_server_protocol::AppsListParams;
use bgs_app_server_protocol::AppsListResponse;
use bgs_app_server_protocol::AppsReadParams;
use bgs_app_server_protocol::AppsReadResponse;
use bgs_app_server_protocol::AskForApproval;
use bgs_app_server_protocol::AuthMode;
use bgs_app_server_protocol::CancelLoginAccountParams;
use bgs_app_server_protocol::CancelLoginAccountResponse;
use bgs_app_server_protocol::CancelLoginAccountStatus;
use bgs_app_server_protocol::ClientInfo;
use bgs_app_server_protocol::ClientRequest;
use bgs_app_server_protocol::ClientResponsePayload;
use bgs_app_server_protocol::BgsErrorInfo;
use bgs_app_server_protocol::CollaborationModeListParams;
use bgs_app_server_protocol::CollaborationModeListResponse;
use bgs_app_server_protocol::CommandExecParams;
use bgs_app_server_protocol::CommandExecResizeParams;
use bgs_app_server_protocol::CommandExecTerminateParams;
use bgs_app_server_protocol::CommandExecWriteParams;
use bgs_app_server_protocol::ConfigWarningNotification;
use bgs_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use bgs_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use bgs_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use bgs_app_server_protocol::ConversationGitInfo;
use bgs_app_server_protocol::ConversationSummary;
use bgs_app_server_protocol::DeprecationNoticeNotification;
use bgs_app_server_protocol::DynamicToolFunctionSpec;
use bgs_app_server_protocol::DynamicToolNamespaceTool;
use bgs_app_server_protocol::DynamicToolSpec;
use bgs_app_server_protocol::EnvironmentAddParams;
use bgs_app_server_protocol::EnvironmentAddResponse;
use bgs_app_server_protocol::EnvironmentInfoParams;
use bgs_app_server_protocol::EnvironmentInfoResponse;
use bgs_app_server_protocol::EnvironmentShellInfo;
use bgs_app_server_protocol::EnvironmentStatusKind;
use bgs_app_server_protocol::EnvironmentStatusParams;
use bgs_app_server_protocol::EnvironmentStatusResponse;
use bgs_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use bgs_app_server_protocol::ExperimentalFeatureListParams;
use bgs_app_server_protocol::ExperimentalFeatureListResponse;
use bgs_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use bgs_app_server_protocol::FeedbackUploadParams;
use bgs_app_server_protocol::FeedbackUploadResponse;
use bgs_app_server_protocol::GetAccountParams;
use bgs_app_server_protocol::GetAccountRateLimitsResponse;
use bgs_app_server_protocol::GetAccountResponse;
use bgs_app_server_protocol::GetAccountTokenUsageParams;
use bgs_app_server_protocol::GetAccountTokenUsageResponse;
use bgs_app_server_protocol::GetAuthStatusParams;
use bgs_app_server_protocol::GetAuthStatusResponse;
use bgs_app_server_protocol::GetConversationSummaryParams;
use bgs_app_server_protocol::GetConversationSummaryResponse;
use bgs_app_server_protocol::GetWorkspaceMessagesResponse;
use bgs_app_server_protocol::GitDiffToRemoteParams;
use bgs_app_server_protocol::GitDiffToRemoteResponse;
use bgs_app_server_protocol::GitInfo as ApiGitInfo;
use bgs_app_server_protocol::HookHandlerMetadata;
use bgs_app_server_protocol::HookMetadata;
use bgs_app_server_protocol::HooksListParams;
use bgs_app_server_protocol::HooksListResponse;
use bgs_app_server_protocol::InitializeParams;
use bgs_app_server_protocol::InitializeResponse;
use bgs_app_server_protocol::InstalledApp;
use bgs_app_server_protocol::JSONRPCErrorError;
use bgs_app_server_protocol::ListMcpServerStatusParams;
use bgs_app_server_protocol::ListMcpServerStatusResponse;
use bgs_app_server_protocol::LoginAccountParams;
use bgs_app_server_protocol::LoginAccountResponse;
use bgs_app_server_protocol::LoginApiKeyParams;
use bgs_app_server_protocol::LoginAppBrand;
use bgs_app_server_protocol::LogoutAccountResponse;
use bgs_app_server_protocol::MarketplaceAddParams;
use bgs_app_server_protocol::MarketplaceAddResponse;
use bgs_app_server_protocol::MarketplaceInterface;
use bgs_app_server_protocol::MarketplaceRemoveParams;
use bgs_app_server_protocol::MarketplaceRemoveResponse;
use bgs_app_server_protocol::MarketplaceUpgradeErrorInfo;
use bgs_app_server_protocol::MarketplaceUpgradeParams;
use bgs_app_server_protocol::MarketplaceUpgradeResponse;
use bgs_app_server_protocol::McpResourceReadParams;
use bgs_app_server_protocol::McpResourceReadResponse;
use bgs_app_server_protocol::McpServerOauthClientRegistration;
use bgs_app_server_protocol::McpServerOauthLoginCompletedNotification;
use bgs_app_server_protocol::McpServerOauthLoginParams;
use bgs_app_server_protocol::McpServerOauthLoginResponse;
use bgs_app_server_protocol::McpServerRefreshResponse;
use bgs_app_server_protocol::McpServerStatus;
use bgs_app_server_protocol::McpServerStatusDetail;
use bgs_app_server_protocol::McpServerToolCallParams;
use bgs_app_server_protocol::McpServerToolCallResponse;
use bgs_app_server_protocol::MemoryResetResponse;
use bgs_app_server_protocol::MockExperimentalMethodParams;
use bgs_app_server_protocol::MockExperimentalMethodResponse;
use bgs_app_server_protocol::ModelListParams;
use bgs_app_server_protocol::ModelListResponse;
use bgs_app_server_protocol::PermissionProfileListParams;
use bgs_app_server_protocol::PermissionProfileListResponse;
use bgs_app_server_protocol::PermissionProfileSummary;
use bgs_app_server_protocol::PluginDetail;
use bgs_app_server_protocol::PluginInstallParams;
use bgs_app_server_protocol::PluginInstallResponse;
use bgs_app_server_protocol::PluginInstalledParams;
use bgs_app_server_protocol::PluginInstalledResponse;
use bgs_app_server_protocol::PluginInterface;
use bgs_app_server_protocol::PluginListMarketplaceKind;
use bgs_app_server_protocol::PluginListParams;
use bgs_app_server_protocol::PluginListResponse;
use bgs_app_server_protocol::PluginMarketplaceEntry;
use bgs_app_server_protocol::PluginReadParams;
use bgs_app_server_protocol::PluginReadResponse;
use bgs_app_server_protocol::PluginShareCheckoutParams;
use bgs_app_server_protocol::PluginShareCheckoutResponse;
use bgs_app_server_protocol::PluginShareContext;
use bgs_app_server_protocol::PluginShareDeleteParams;
use bgs_app_server_protocol::PluginShareDeleteResponse;
use bgs_app_server_protocol::PluginShareDiscoverability;
use bgs_app_server_protocol::PluginShareListItem;
use bgs_app_server_protocol::PluginShareListParams;
use bgs_app_server_protocol::PluginShareListResponse;
use bgs_app_server_protocol::PluginSharePrincipal;
use bgs_app_server_protocol::PluginSharePrincipalType;
use bgs_app_server_protocol::PluginShareSaveParams;
use bgs_app_server_protocol::PluginShareSaveResponse;
use bgs_app_server_protocol::PluginShareTarget;
use bgs_app_server_protocol::PluginShareUpdateDiscoverability;
use bgs_app_server_protocol::PluginShareUpdateTargetsParams;
use bgs_app_server_protocol::PluginShareUpdateTargetsResponse;
use bgs_app_server_protocol::PluginSkillReadParams;
use bgs_app_server_protocol::PluginSkillReadResponse;
use bgs_app_server_protocol::PluginSource;
use bgs_app_server_protocol::PluginSummary;
use bgs_app_server_protocol::PluginUninstallParams;
use bgs_app_server_protocol::PluginUninstallResponse;
use bgs_app_server_protocol::RateLimitResetCredit;
use bgs_app_server_protocol::RateLimitResetCreditStatus;
use bgs_app_server_protocol::RateLimitResetCreditsSummary;
use bgs_app_server_protocol::RateLimitResetType;
use bgs_app_server_protocol::RequestId;
use bgs_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use bgs_app_server_protocol::ReviewStartParams;
use bgs_app_server_protocol::ReviewStartResponse;
use bgs_app_server_protocol::ReviewTarget as ApiReviewTarget;
use bgs_app_server_protocol::SandboxMode;
use bgs_app_server_protocol::SendAddCreditsNudgeEmailParams;
use bgs_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use bgs_app_server_protocol::ServerNotification;
use bgs_app_server_protocol::ServerRequestResolvedNotification;
use bgs_app_server_protocol::SkillSummary;
use bgs_app_server_protocol::SkillsConfigWriteParams;
use bgs_app_server_protocol::SkillsConfigWriteResponse;
use bgs_app_server_protocol::SkillsExtraRootsSetParams;
use bgs_app_server_protocol::SkillsExtraRootsSetResponse;
use bgs_app_server_protocol::SkillsListParams;
use bgs_app_server_protocol::SkillsListResponse;
use bgs_app_server_protocol::SortDirection;
use bgs_app_server_protocol::Thread;
use bgs_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use bgs_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use bgs_app_server_protocol::ThreadArchiveParams;
use bgs_app_server_protocol::ThreadArchiveResponse;
use bgs_app_server_protocol::ThreadArchivedNotification;
use bgs_app_server_protocol::ThreadBackgroundTerminal;
use bgs_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use bgs_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use bgs_app_server_protocol::ThreadBackgroundTerminalsListParams;
use bgs_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use bgs_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use bgs_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use bgs_app_server_protocol::ThreadClosedNotification;
use bgs_app_server_protocol::ThreadCompactStartParams;
use bgs_app_server_protocol::ThreadCompactStartResponse;
use bgs_app_server_protocol::ThreadDecrementElicitationParams;
use bgs_app_server_protocol::ThreadDecrementElicitationResponse;
use bgs_app_server_protocol::ThreadDeleteParams;
use bgs_app_server_protocol::ThreadDeleteResponse;
use bgs_app_server_protocol::ThreadDeletedNotification;
use bgs_app_server_protocol::ThreadForkParams;
use bgs_app_server_protocol::ThreadForkResponse;
use bgs_app_server_protocol::ThreadGoal;
use bgs_app_server_protocol::ThreadGoalClearParams;
use bgs_app_server_protocol::ThreadGoalClearResponse;
use bgs_app_server_protocol::ThreadGoalClearedNotification;
use bgs_app_server_protocol::ThreadGoalGetParams;
use bgs_app_server_protocol::ThreadGoalGetResponse;
use bgs_app_server_protocol::ThreadGoalSetParams;
use bgs_app_server_protocol::ThreadGoalSetResponse;
use bgs_app_server_protocol::ThreadGoalStatus;
use bgs_app_server_protocol::ThreadGoalUpdatedNotification;
use bgs_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use bgs_app_server_protocol::ThreadHistoryMode;
use bgs_app_server_protocol::ThreadIncrementElicitationParams;
use bgs_app_server_protocol::ThreadIncrementElicitationResponse;
use bgs_app_server_protocol::ThreadInjectItemsParams;
use bgs_app_server_protocol::ThreadInjectItemsResponse;
use bgs_app_server_protocol::ThreadItem;
use bgs_app_server_protocol::ThreadItemEntry;
use bgs_app_server_protocol::ThreadItemsListParams;
use bgs_app_server_protocol::ThreadItemsListResponse;
use bgs_app_server_protocol::ThreadItemsReadParams;
use bgs_app_server_protocol::ThreadItemsReadResponse;
use bgs_app_server_protocol::ThreadListCwdFilter;
use bgs_app_server_protocol::ThreadListParams;
use bgs_app_server_protocol::ThreadListResponse;
use bgs_app_server_protocol::ThreadLoadedListParams;
use bgs_app_server_protocol::ThreadLoadedListResponse;
use bgs_app_server_protocol::ThreadMemoryModeSetParams;
use bgs_app_server_protocol::ThreadMemoryModeSetResponse;
use bgs_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use bgs_app_server_protocol::ThreadMetadataUpdateParams;
use bgs_app_server_protocol::ThreadMetadataUpdateResponse;
use bgs_app_server_protocol::ThreadNameUpdatedNotification;
use bgs_app_server_protocol::ThreadProjectUpdatedNotification;
use bgs_app_server_protocol::ThreadReadParams;
use bgs_app_server_protocol::ThreadReadResponse;
use bgs_app_server_protocol::ThreadRealtimeAppendAudioParams;
use bgs_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use bgs_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use bgs_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use bgs_app_server_protocol::ThreadRealtimeAppendTextParams;
use bgs_app_server_protocol::ThreadRealtimeAppendTextResponse;
use bgs_app_server_protocol::ThreadRealtimeListVoicesResponse;
use bgs_app_server_protocol::ThreadRealtimeStartParams;
use bgs_app_server_protocol::ThreadRealtimeStartResponse;
use bgs_app_server_protocol::ThreadRealtimeStartTransport;
use bgs_app_server_protocol::ThreadRealtimeStopParams;
use bgs_app_server_protocol::ThreadRealtimeStopResponse;
use bgs_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use bgs_app_server_protocol::ThreadResumeParams;
use bgs_app_server_protocol::ThreadResumeResponse;
use bgs_app_server_protocol::ThreadSearchOccurrence;
use bgs_app_server_protocol::ThreadSearchOccurrencesParams;
use bgs_app_server_protocol::ThreadSearchOccurrencesResponse;
use bgs_app_server_protocol::ThreadSearchParams;
use bgs_app_server_protocol::ThreadSearchResponse;
use bgs_app_server_protocol::ThreadSearchResult;
use bgs_app_server_protocol::ThreadSearchSortKey;
use bgs_app_server_protocol::ThreadSearchTextRange;
use bgs_app_server_protocol::ThreadSetNameParams;
use bgs_app_server_protocol::ThreadSetNameResponse;
use bgs_app_server_protocol::ThreadSettings;
use bgs_app_server_protocol::ThreadSettingsUpdateParams;
use bgs_app_server_protocol::ThreadSettingsUpdateResponse;
use bgs_app_server_protocol::ThreadShellCommandParams;
use bgs_app_server_protocol::ThreadShellCommandResponse;
use bgs_app_server_protocol::ThreadSortKey;
use bgs_app_server_protocol::ThreadSourceKind;
use bgs_app_server_protocol::ThreadStartParams;
use bgs_app_server_protocol::ThreadStartResponse;
use bgs_app_server_protocol::ThreadStartedNotification;
use bgs_app_server_protocol::ThreadStatus;
use bgs_app_server_protocol::ThreadTimelineListParams;
use bgs_app_server_protocol::ThreadTimelineListResponse;
use bgs_app_server_protocol::ThreadTurnsListParams;
use bgs_app_server_protocol::ThreadTurnsListResponse;
use bgs_app_server_protocol::ThreadUnarchiveParams;
use bgs_app_server_protocol::ThreadUnarchiveResponse;
use bgs_app_server_protocol::ThreadUnarchivedNotification;
use bgs_app_server_protocol::ThreadUnsubscribeParams;
use bgs_app_server_protocol::ThreadUnsubscribeResponse;
use bgs_app_server_protocol::ThreadUnsubscribeStatus;
use bgs_app_server_protocol::Turn;
use bgs_app_server_protocol::TurnEnvironmentParams;
use bgs_app_server_protocol::TurnError;
use bgs_app_server_protocol::TurnInterruptParams;
use bgs_app_server_protocol::TurnInterruptResponse;
use bgs_app_server_protocol::TurnItemsView;
use bgs_app_server_protocol::TurnSettingsUpdateParams;
use bgs_app_server_protocol::TurnSettingsUpdateResponse;
use bgs_app_server_protocol::TurnSettingsUpdateStatus;
use bgs_app_server_protocol::TurnStartParams;
use bgs_app_server_protocol::TurnStartResponse;
use bgs_app_server_protocol::TurnStatus;
use bgs_app_server_protocol::TurnSteerParams;
use bgs_app_server_protocol::TurnSteerResponse;
use bgs_app_server_protocol::UserInput as V2UserInput;
use bgs_app_server_protocol::WindowsSandboxReadiness;
use bgs_app_server_protocol::WindowsSandboxReadinessResponse;
use bgs_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use bgs_app_server_protocol::WindowsSandboxSetupMode;
use bgs_app_server_protocol::WindowsSandboxSetupStartParams;
use bgs_app_server_protocol::WindowsSandboxSetupStartResponse;
use bgs_app_server_protocol::WorkspaceMessage;
use bgs_app_server_protocol::WorkspaceMessageType;
use bgs_arg0::Arg0DispatchPaths;
use bgs_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use bgs_backend_client::Client as BackendClient;
use bgs_backend_client::BgsWorkspaceMessage as BackendWorkspaceMessage;
use bgs_backend_client::BgsWorkspaceMessageType as BackendWorkspaceMessageType;
use bgs_backend_client::BgsWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use bgs_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use bgs_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use bgs_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use bgs_backend_client::RequestError as BackendRequestError;
use bgs_backend_client::TokenUsageProfile;
use bgs_chatgpt::connectors;
use bgs_config::CloudConfigBundleLoadError;
use bgs_config::CloudConfigBundleLoadErrorCode;
use bgs_config::ConfigLayerStack;
use bgs_config::loader::project_trust_key;
use bgs_config::types::McpServerTransportConfig;
use bgs_connectors::AppInfo;
use bgs_core::BgsThread;
use bgs_core::BgsThreadSettingsOverrides;
use bgs_core::ForkSnapshot;
use bgs_core::McpManager;
use bgs_core::NewThread;
use bgs_core::NotSubmittedReason;
#[cfg(test)]
use bgs_core::SessionMeta;
use bgs_core::StartThreadOptions;
use bgs_core::SteerSubmission;
use bgs_core::ThreadConfigSnapshot;
use bgs_core::ThreadManager;
use bgs_core::TurnInput;
use bgs_core::TurnInputRequest;
use bgs_core::TurnInputSubmission;
use bgs_core::TurnStartOptions;
use bgs_core::config::Config;
use bgs_core::config::ConfigOverrides;
use bgs_core::config::NetworkProxyAuditMetadata;
use bgs_core::config::edit::ConfigEdit;
use bgs_core::config::edit::ConfigEditsBuilder;
use bgs_core::connectors::AccessibleConnectorsStatus;
use bgs_core::exec::ExecCapturePolicy;
use bgs_core::exec::ExecExpiration;
use bgs_core::exec::ExecParams;
use bgs_core::exec_env::create_env;
use bgs_core::path_utils;
#[cfg(test)]
use bgs_core::read_head_for_summary;
use bgs_core::sandboxing::SandboxPermissions;
use bgs_core::truncate_rollout_after_turn_id;
use bgs_core::truncate_rollout_before_turn_id;
use bgs_core::validate_environment_ids_and_cwds;
use bgs_core::windows_sandbox::WindowsSandboxLevelExt;
use bgs_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use bgs_core::windows_sandbox::WindowsSandboxSetupRequest;
use bgs_core::windows_sandbox::sandbox_setup_is_complete;
use bgs_core_plugins::PluginInstallError as CorePluginInstallError;
use bgs_core_plugins::PluginInstallRequest;
use bgs_core_plugins::PluginReadRequest;
use bgs_core_plugins::PluginUninstallError as CorePluginUninstallError;
use bgs_core_plugins::PluginsManager;
use bgs_core_plugins::loader::load_plugin_apps;
use bgs_core_plugins::manifest::PluginManifestInterface;
use bgs_core_plugins::marketplace::MarketplaceError;
use bgs_core_plugins::marketplace::MarketplacePluginSource;
use bgs_core_plugins::marketplace_add::MarketplaceAddError;
use bgs_core_plugins::marketplace_add::MarketplaceAddRequest;
use bgs_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_bgs_home;
use bgs_core_plugins::marketplace_remove::MarketplaceRemoveError;
use bgs_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use bgs_core_plugins::marketplace_remove::remove_marketplace;
use bgs_core_plugins::remote::RemoteMarketplace;
use bgs_core_plugins::remote::RemoteMarketplaceSource;
use bgs_core_plugins::remote::RemotePluginCatalogError;
use bgs_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use bgs_core_plugins::remote::RemotePluginServiceConfig;
use bgs_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use bgs_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use bgs_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use bgs_exec_server::EnvironmentManager;
use bgs_exec_server::EnvironmentObservedStatus;
use bgs_exec_server::LOCAL_ENVIRONMENT_ID;
use bgs_exec_server::LOCAL_FS;
use bgs_features::FEATURES;
use bgs_features::Feature;
use bgs_features::Stage;
use bgs_feedback::BgsFeedback;
use bgs_feedback::FeedbackAttachmentPath;
use bgs_feedback::FeedbackUploadOptions;
use bgs_git_utils::git_diff_to_remote;
use bgs_git_utils::resolve_root_git_project_for_trust;
use bgs_login::AuthManager;
use bgs_login::BGS_OPEN_APP_URL;
use bgs_login::BgsAuth;
use bgs_login::LoginSuccessPage;
use bgs_login::LoginSuccessPageBrand;
use bgs_login::ServerOptions as LoginServerOptions;
use bgs_login::ShutdownHandle;
use bgs_login::complete_device_code_login;
use bgs_login::login_with_api_key;
use bgs_login::login_with_bedrock_api_key;
use bgs_login::oauth_client_id;
use bgs_login::request_device_code;
use bgs_login::run_login_server;
use bgs_mcp::McpRuntimeContext;
use bgs_mcp::McpServerStatusSnapshot;
use bgs_mcp::McpSnapshotDetail;
use bgs_mcp::collect_mcp_server_status_snapshot_with_detail;
use bgs_mcp::discover_supported_scopes;
use bgs_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use bgs_mcp::resolve_oauth_scopes;
use bgs_memories_write::clear_memory_roots_contents;
use bgs_model_provider::create_model_provider;
use bgs_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use bgs_protocol::ThreadId;
use bgs_protocol::config_types::CollaborationMode;
use bgs_protocol::config_types::ForcedLoginMethod;
use bgs_protocol::config_types::Personality;
use bgs_protocol::config_types::ReasoningSummary;
use bgs_protocol::config_types::TrustLevel;
use bgs_protocol::config_types::WindowsSandboxLevel;
use bgs_protocol::error::BgsErr;
use bgs_protocol::error::Result as BgsResult;
#[cfg(test)]
use bgs_protocol::items::TurnItem;
use bgs_protocol::models::ResponseItem;
use bgs_protocol::openai_models::ReasoningEffort;
use bgs_protocol::protocol::AgentStatus;
use bgs_protocol::protocol::ConversationAudioParams;
use bgs_protocol::protocol::ConversationSpeechParams;
use bgs_protocol::protocol::ConversationStartParams;
use bgs_protocol::protocol::ConversationStartTransport;
use bgs_protocol::protocol::ConversationTextParams;
use bgs_protocol::protocol::EnvironmentConfigState;
use bgs_protocol::protocol::EventMsg;
#[cfg(test)]
use bgs_protocol::protocol::GitInfo as CoreGitInfo;
use bgs_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use bgs_protocol::protocol::Op;
use bgs_protocol::protocol::RealtimeVoicesList;
use bgs_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use bgs_protocol::protocol::ReviewRequest;
use bgs_protocol::protocol::ReviewTarget as CoreReviewTarget;
use bgs_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use bgs_protocol::protocol::SessionMetaLine;
use bgs_protocol::protocol::TurnEnvironmentRequest;
use bgs_protocol::protocol::W3cTraceContext;
use bgs_protocol::protocol::strip_user_message_prefix;
use bgs_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use bgs_protocol::user_input::UserInput as CoreInputItem;
use bgs_rmcp_client::McpOAuthClientRegistration;
use bgs_rmcp_client::StreamableHttpRedirectMode;
use bgs_rmcp_client::perform_oauth_login_return_url;
use bgs_rollout::InitialHistory;
use bgs_rollout::ResumedHistory;
use bgs_rollout::RolloutItem;
use bgs_rollout::persisted_rollout_item;
use bgs_rollout::state_db::StateDbHandle;
use bgs_rollout::state_db::reconcile_rollout;
use bgs_state::ThreadMetadata;
use bgs_state::log_db::LogDbLayer;
use bgs_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use bgs_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use bgs_thread_store::ClearableField as StoreClearableField;
use bgs_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use bgs_thread_store::GitInfoPatch as StoreGitInfoPatch;
use bgs_thread_store::ItemSortKey as StoreItemSortKey;
use bgs_thread_store::ListItemsParams as StoreListItemsParams;
use bgs_thread_store::ListItemsPosition as StoreListItemsPosition;
use bgs_thread_store::ListThreadsParams as StoreListThreadsParams;
use bgs_thread_store::ListTimelineParams as StoreListTimelineParams;
use bgs_thread_store::ListTurnsParams as StoreListTurnsParams;
use bgs_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use bgs_thread_store::LocalThreadStore;
use bgs_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use bgs_thread_store::ReadThreadParams as StoreReadThreadParams;
use bgs_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use bgs_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use bgs_thread_store::SortDirection as StoreSortDirection;
use bgs_thread_store::StoredThread;
use bgs_thread_store::StoredTurn;
use bgs_thread_store::StoredTurnItemsView;
use bgs_thread_store::StoredTurnStatus;
use bgs_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use bgs_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use bgs_thread_store::ThreadSortKey as StoreThreadSortKey;
use bgs_thread_store::ThreadStore;
use bgs_thread_store::ThreadStoreError;
use bgs_utils_absolute_path::AbsolutePathBuf;
use bgs_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use bgs_app_server_protocol::ServerRequest;

pub(crate) mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
pub(crate) mod config_processor;
#[cfg(test)]
#[path = "request_processors/config_reload_tests.rs"]
mod config_reload_tests;
mod diagnostics;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod feedback_rollout_history;
mod feedback_thread_index;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_event_stream;
mod mcp_processor;
mod memory_status;
mod persisted_resume_settings;
mod plugins;
mod process_exec_processor;
mod projects;
mod remote_control_processor;
mod rollout;
mod search;
mod thread_attachments;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_input;
mod thread_processor;
mod thread_queue_processor;
mod thread_read_state;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use diagnostics::read_server_diagnostics;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_event_stream::McpEventStreamReady;
pub(crate) use mcp_event_stream::McpEventStreams;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use projects::ProjectRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use thread_processor::ThreadResumeTarget;
pub(crate) use thread_queue_processor::ThreadQueueRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::PendingThreadUnloads;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

pub(crate) fn apply_live_thread_settings(
    thread: &mut Thread,
    config_snapshot: &ThreadConfigSnapshot,
) {
    thread.model = Some(config_snapshot.model.clone());
    thread.reasoning_effort = config_snapshot.reasoning_effort.clone();
    thread.environments = Some(
        config_snapshot
            .environment_selections()
            .iter()
            .map(Into::into)
            .collect(),
    );
}

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_requests(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentRequest>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut requests = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        requests.push(TurnEnvironmentRequest {
            environment_id,
            cwd,
            workspace_roots,
            config: EnvironmentConfigState::FromThread,
        });
    }
    validate_environment_ids_and_cwds(
        &thread_manager.environment_manager(),
        requests
            .iter()
            .map(|request| (request.environment_id.as_str(), &request.cwd)),
    )
    .map_err(environment_selection_error)?;
    Ok(Some(requests))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

pub(crate) mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if let Some(item) =
            persisted_rollout_item(item, bgs_protocol::protocol::ThreadHistoryMode::Legacy)
        {
            builder.handle_rollout_item(item.as_ref());
        }
    }
    builder.finish()
}
