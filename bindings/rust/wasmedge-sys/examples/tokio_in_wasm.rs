use wasmedge_sys::{Config, Vm};

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = std::path::Path::new("/Users/sam/workspace/rust/async-demo/run-tokio-in-wasm/target/wasm32-wasi/release/run_tokio_in_wasm.wasm");

    // create a Config context
    let mut config = Config::create()?;
    config.bulk_memory_operations(true);
    config.wasi(true);
    assert!(config.wasi_enabled());

    let vm = Vm::create(Some(config), None)?;
    vm.run_wasm_from_file(wasm_file, "hello", [])?;

    Ok(())
}
