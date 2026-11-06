use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_bgs_access_token("at-example"),
        BgsAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_bgs_access_token("header.payload.signature"),
        BgsAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
