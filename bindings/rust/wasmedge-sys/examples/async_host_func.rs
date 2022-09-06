use wasmedge_macro::sys_async_host_function;
use wasmedge_sys::{CallingFrame, Executor, FuncType, Function, WasmValue};
use wasmedge_types::error::HostFuncError;

// Native async function
#[sys_async_host_function]
async fn say_hello(
    _frame: &CallingFrame,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");
    Ok::<Vec<WasmValue>, HostFuncError>(vec![])
}

// // native async func wrapper
// fn wrap_say_hello<'a>(
//     frame: &'a CallingFrame,
//     input: Vec<WasmValue>,
// ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<WasmValue>, HostFuncError>> + 'a>>
// {
//     Box::pin(say_hello(&frame, input))
// }

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a FuncType
    let func_ty = FuncType::create(vec![], vec![])?;

    // create a host function
    let async_host_func = Function::create_async(&func_ty, Box::new(say_hello), 0)?;

    // run this function
    let mut executor = Executor::create(None, None)?;
    async_host_func.call(&mut executor, vec![])?;

    Ok(())
}
