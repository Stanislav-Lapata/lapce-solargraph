use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("ruby")),
        pattern: Some(String::from("**/*.rb")),
        scheme: None,
    }];
    let mut server_args = vec![String::from("stdio")];

    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(args) = options.get("serverArgs") {
            if let Some(args) = args.as_array() {
                if !args.is_empty() {
                    server_args = vec![];
                }
                for arg in args {
                    if let Some(arg) = arg.as_str() {
                        server_args.push(arg.to_string());
                    }
                }
            }
        }

        if let Some(server_path) = options.get("serverPath") {
            if let Some(server_path) = server_path.as_str() {
                if !server_path.is_empty() {
                    let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                    PLUGIN_RPC.start_lsp(
                        server_uri,
                        server_args,
                        document_selector,
                        params.initialization_options,
                    );
                    return Ok(());
                }
            }
        }
    }

    let server_uri = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => return Ok(()),
        _ => Url::parse("urn:solargraph")?,
    };

    PLUGIN_RPC.start_lsp(
        server_uri,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
