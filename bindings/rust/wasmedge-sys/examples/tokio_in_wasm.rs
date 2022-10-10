//! Use the following command to run this example:
//!
//! ```bash
//! cd bindings/rust
//! cargo run -p wasmedge-sys --example tokio_in_wasm -- --nocapture
//! ```
//!

use wasmedge_sys::{Config, Vm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = std::path::PathBuf::from(env!("WASMEDGE_DIR"))
        .join("bindings/rust/wasmedge-sys/examples/data/run_tokio_in_wasm.wasm");

    // create a Config context
    let mut config = Config::create()?;
    config.bulk_memory_operations(true);
    config.wasi(true);
    assert!(config.wasi_enabled());

    let mut vm = Vm::create(Some(config), None)?;

    // initialize the wasi module
    let mut wasi_module = vm.wasi_module_mut()?;
    wasi_module.init_wasi(None, None, None);

    // run host function
    vm.run_wasm_from_file(wasm_file, "hello", [])?;

    Ok(())
}
