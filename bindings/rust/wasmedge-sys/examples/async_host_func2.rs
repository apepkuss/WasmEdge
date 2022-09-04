use switcher2::stack::*;
use wasmedge_sys::{
    async_env::AsyncWasmEdgeResult, CallingFrame, Executor, FuncType, Function,
    WasmEdgeHostFuncFuture, WasmEdgeHostFuncResult, WasmValue,
};

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Native async function
    fn async_say_hello(_: &CallingFrame, _: Vec<WasmValue>) -> WasmEdgeHostFuncFuture {
        let stack = EightMbStack::new().unwrap();
        AsyncWasmEdgeResult::<EightMbStack, WasmEdgeHostFuncResult<Vec<WasmValue>>, fn()>::new(
            stack,
            |_| {
                println!("Hello, world!");
                Ok(vec![])
            },
        )
        .unwrap()
    }

    // create a FuncType
    let func_ty = FuncType::create(vec![], vec![])?;

    // create a host function
    let async_host_func = Function::create_async_new(&func_ty, Box::new(async_say_hello), 0)?;

    // run this function
    let mut executor = Executor::create(None, None)?;
    async_host_func.call(&mut executor, vec![])?;

    Ok(())
}
