use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;

fn main() {
    let mut io = IoHandler::default();

    io.add_method("ku_clientVersion", |_params: Params| async {
        let kusari_version = env!("CARGO_PKG_VERSION");
        let commit_hash = &env!("COMMIT_HASH")[0..7];
        let os_type = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let rustc_version = env!("RUSTC_VERSION");

        let result = format!(
            "kusari/v{kusari_version}-stable-{commit_hash}/{os_type}-{arch}/rust{rustc_version}"
        );

        Ok(Value::String(result))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
