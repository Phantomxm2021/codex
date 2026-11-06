const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum BgsAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_bgs_access_token(access_token: &str) -> BgsAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        BgsAccessToken::PersonalAccessToken(access_token)
    } else {
        BgsAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
