use wasmedge_sys::{Vm, WasmValue};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_file = std::path::Path::new(
        "/Volumes/Dev/secondstate/me/issue/issue-1580/target/wasm32-wasi/release/test.wasm",
    );

    let mut vm = Vm::create(None, None)?;
    vm.register_wasm_from_file("extern", wasm_file)?;

    let async_result = vm.run_registered_function_async_old(
        "extern",
        "plus_one",
        [WasmValue::from_i32(10), WasmValue::from_i32(99)],
    )?;

    // get the result returned by the host function
    let returns = async_result.get_async()?;
    assert_eq!(returns.len(), 1);

    Ok(())
}
