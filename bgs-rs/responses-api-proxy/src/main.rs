use clap::Parser;
use bgs_responses_api_proxy::Args as ResponsesApiProxyArgs;

#[ctor::ctor]
fn pre_main() {
    bgs_process_hardening::pre_main_hardening();
}

pub fn main() -> anyhow::Result<()> {
    let args = ResponsesApiProxyArgs::parse();
    bgs_responses_api_proxy::run_main(args)
}
