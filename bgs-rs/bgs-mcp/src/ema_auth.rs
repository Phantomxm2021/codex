//! Derives the enterprise credential owner from the active Bgs account.

use bgs_config::McpEmaAuthScope;
use bgs_login::BgsAuth;

pub fn ema_auth_scope(auth: Option<&BgsAuth>) -> Option<McpEmaAuthScope> {
    let auth = auth?;
    McpEmaAuthScope::new(auth.get_chatgpt_user_id()?, auth.get_account_id()?)
}
