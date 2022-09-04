use wasmedge_sys::{CallingFrame, Executor, FuncType, Function, WasmValue};
use wasmedge_types::error::HostFuncError;

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Native async function
    async fn say_hello(
        _: &CallingFrame,
        _: Vec<WasmValue>,
    ) -> Result<Vec<WasmValue>, HostFuncError> {
        println!("Hello, world!");
        Ok::<Vec<WasmValue>, HostFuncError>(vec![])
    }

    // native async func wrapper
    fn wrap_say_hello<'a>(
        frame: &'a CallingFrame,
        input: Vec<WasmValue>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<WasmValue>, HostFuncError>> + 'a>,
    > {
        Box::pin(say_hello(&frame, input))
    }

    // create a FuncType
    let func_ty = FuncType::create(vec![], vec![])?;

    // create a host function
    let async_host_func = Function::create_async(&func_ty, Box::new(wrap_say_hello), 0)?;

    // run this function
    let mut executor = Executor::create(None, None)?;
    async_host_func.call(&mut executor, vec![])?;

    Ok(())
}
