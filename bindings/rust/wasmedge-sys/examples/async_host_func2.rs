use switcher2::stack::*;
use wasmedge_macro::sys_async_host_function2;
use wasmedge_sys::{
    async_env::AsyncWasmEdgeResult, CallingFrame, Executor, FuncType, Function,
    WasmEdgeHostFuncFuture, WasmEdgeHostFuncResult, WasmValue,
};
use wasmedge_types::error::HostFuncError;

// Native async function
#[sys_async_host_function2]
async fn say_hello(
    _frame: &CallingFrame,
    _args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Hello, world!");
    Ok::<Vec<WasmValue>, HostFuncError>(vec![])
}

// // Native async function
// fn async_say_hello(frame: &CallingFrame, args: Vec<WasmValue>) -> WasmEdgeHostFuncFuture {
//     let stack = EightMbStack::new().unwrap();
//     AsyncWasmEdgeResult::<EightMbStack, WasmEdgeHostFuncResult<Vec<WasmValue>>, fn()>::new(
//         stack,
//         |mut yielder| -> Result<Vec<WasmValue>, HostFuncError> {
//             yielder.async_suspend(say_hello(frame, args))
//         },
//     )
//     .unwrap()
// }

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a FuncType
    let func_ty = FuncType::create(vec![], vec![])?;

    // create a host function
    let async_host_func = Function::create_async_new(&func_ty, Box::new(say_hello), 0)?;

    // run this function
    let mut executor = Executor::create(None, None)?;
    async_host_func.call(&mut executor, vec![])?;

    Ok(())
}
