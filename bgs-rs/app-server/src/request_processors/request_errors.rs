use super::*;
use bgs_protocol::error::BgsErrorDetails;

pub(super) fn environment_selection_error(err: BgsErr) -> JSONRPCErrorError {
    match err.details() {
        BgsErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
