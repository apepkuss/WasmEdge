use wasmedge_sys::{Config, Store, Vm, WasmValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a Config context
    let result = Config::create();
    assert!(result.is_ok());
    let mut config = result.unwrap();
    config.bulk_memory_operations(true);
    assert!(config.bulk_memory_operations_enabled());

    // create a Store context
    let result = Store::create();
    assert!(result.is_ok(), "Failed to create Store instance");
    let mut store = result.unwrap();

    // create a Vm context with the given Config and Store
    let result = Vm::create(Some(config), Some(&mut store));
    assert!(result.is_ok());
    let mut vm = result.unwrap();

    // register a wasm module from a buffer
    let path = std::path::PathBuf::from(env!("WASMEDGE_DIR"))
        .join("bindings/rust/wasmedge-sys/tests/data/fibonacci.wasm");
    let result = std::fs::read(path);
    assert!(result.is_ok());
    let buffer = result.unwrap();
    let result = vm.register_wasm_from_bytes("extern", &buffer);
    assert!(result.is_ok());

    let fut = async {
        println!("task started");

        // If the duration is set to 1 sec, then will not trigger timeout.
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let _ = vm
            .run_registered_function_async(
                String::from("demo"),
                0,
                "extern",
                "fib",
                vec![WasmValue::from_i32(20)],
            )
            .await;

        println!("task finished");
    };

    let _ = tokio::time::timeout(tokio::time::Duration::from_secs(2), fut).await;
    println!("done");

    Ok(())
}
