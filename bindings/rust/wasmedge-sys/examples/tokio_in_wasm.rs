use wasmedge_sys::Vm;

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = std::path::Path::new("/Users/sam/workspace/rust/async-demo/run-tokio-in-wasm/target/wasm32-wasi/release/run_tokio_in_wasm.wasm");

    let vm = Vm::create(None, None)?;
    vm.run_wasm_from_file(wasm_file, "hello", [])?;

    Ok(())
}
