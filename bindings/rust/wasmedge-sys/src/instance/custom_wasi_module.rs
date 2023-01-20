use crate::{
    error::{HostFuncError, InstanceError, WasmEdgeError},
    ffi,
    instance::{
        function::{FuncType, InnerFunc},
        global::InnerGlobal,
        memory::InnerMemory,
        module::{AsImport, AsInstance, InnerInstance},
        table::InnerTable,
    },
    types::WasmEdgeString,
    CallingFrame, Function, Global, Memory, Table, WasmEdgeResult, WasmValue, WASI_ENVIRON,
};
use std::{mem::MaybeUninit, sync::Arc};
use wasmedge_types::ValType;

use wasmedge_wasi_common::{Ciovec, CiovecArray, WasiSnapshotPreview1};

#[derive(Debug)]
pub struct CustomWasiModule {
    pub(crate) inner: Arc<InnerInstance>,
    pub(crate) registered: bool,
    pub(crate) name: String,
}
impl Drop for CustomWasiModule {
    fn drop(&mut self) {
        if !self.registered && Arc::strong_count(&self.inner) == 1 && !self.inner.0.is_null() {
            unsafe {
                ffi::WasmEdge_ModuleInstanceDelete(self.inner.0);
            }
        }
    }
}
impl CustomWasiModule {
    /// Creates a WASI host module which contains the WASI host functions, and initializes it with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `args` - The commandline arguments. The first argument is the program name.
    ///
    /// * `envs` - The environment variables in the format `ENV_VAR_NAME=VALUE`.
    ///
    /// * `preopens` - The directories to pre-open. The required format is `DIR1:DIR2`.
    ///
    /// # Error
    ///
    /// If fail to create a host module, then an error is returned.
    pub fn create(
        args: Option<Vec<&str>>,
        envs: Option<Vec<(&str, &str)>>,
        preopened_dirs: Option<Vec<(cap_std::fs::Dir, &std::path::Path)>>,
    ) -> WasmEdgeResult<Self> {
        // create an import module named `wasi_snapshot_preview1`
        let name = "wasi_snapshot_preview1";
        let raw_name = WasmEdgeString::from(name);
        let ctx = unsafe { ffi::WasmEdge_ModuleInstanceCreate(raw_name.as_raw()) };
        if ctx.is_null() {
            return Err(Box::new(WasmEdgeError::Instance(
                InstanceError::CreateImportModule,
            )));
        }

        let mut custom_wasi_module = Self {
            inner: std::sync::Arc::new(InnerInstance(ctx)),
            registered: false,
            name: name.to_string(),
        };

        custom_wasi_module.init(args, envs, preopened_dirs)?;

        Ok(custom_wasi_module)
    }

    /// Initializes the WASI host module with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `args` - The commandline arguments. The first argument is the program name.
    ///
    /// * `envs` - The environment variables in the format `ENV_VAR_NAME=VALUE`.
    ///
    /// * `preopens` - The directories to pre-open. The required format is `DIR1:DIR2`.
    pub fn init(
        &mut self,
        args: Option<Vec<&str>>,
        envs: Option<Vec<(&str, &str)>>,
        preopened_dirs: Option<Vec<(cap_std::fs::Dir, &std::path::Path)>>,
    ) -> WasmEdgeResult<()> {
        let mut global_wasi_environ = WASI_ENVIRON.write();

        // parse arguments
        if let Some(args) = args {
            for arg in args {
                global_wasi_environ.push_arg(arg);
            }
        }
        // parse environment variables
        if let Some(envs) = envs {
            for (var, val) in envs {
                global_wasi_environ.push_env(var, val);
            }
        }
        // parse preopened directories
        if let Some(preopened_dirs) = preopened_dirs {
            for (dir, guest_path) in preopened_dirs {
                let dir = Box::new(wasmedge_wasi::dir::Dir::from_cap_std(dir));
                global_wasi_environ.push_preopened_dir(dir, guest_path);
            }
        }

        // * add wasi host functions

        // `args_sizes_get`
        let ty = FuncType::create(vec![ValType::I32, ValType::I32], vec![ValType::I32])?;
        self.add_func(
            "args_sizes_get",
            Function::create(&ty, Box::new(wasi_args_sizes_get), 0)?,
        );
        // `args_get`
        let ty = FuncType::create(vec![ValType::I32, ValType::I32], vec![ValType::I32])?;
        self.add_func(
            "args_get",
            Function::create(&ty, Box::new(wasi_args_get), 0)?,
        );
        // `environ_sizes_get`
        let ty = FuncType::create(vec![ValType::I32, ValType::I32], vec![ValType::I32])?;
        self.add_func(
            "environ_sizes_get",
            Function::create(&ty, Box::new(wasi_environ_sizes_get), 0)?,
        );
        // `environ_get`
        let ty = FuncType::create(vec![ValType::I32, ValType::I32], vec![ValType::I32])?;
        self.add_func(
            "environ_get",
            Function::create(&ty, Box::new(wasi_environ_get), 0)?,
        );
        // `fd_write`
        let ty = FuncType::create(
            vec![ValType::I32, ValType::I32, ValType::I32, ValType::I32],
            vec![ValType::I32],
        )?;
        self.add_func(
            "fd_write",
            Function::create(&ty, Box::new(wasi_fd_write), 0)?,
        );

        // `proc_exit`
        let ty = FuncType::create(vec![ValType::I32], vec![])?;
        self.add_func(
            "proc_exit",
            Function::create(&ty, Box::new(wasi_proc_exit), 0)?,
        );

        global_wasi_environ.exit_code = 0;

        Ok(())
    }

    pub fn exit_code(&self) -> i32 {
        let global_wasi_environ = WASI_ENVIRON.read();
        global_wasi_environ.exit_code
    }

    /// Returns the native handler from the mapped FD/Handler.
    ///
    /// # Argument
    ///
    /// * `fd` - The WASI mapped Fd.
    ///
    /// # Error
    ///
    /// If fail to get the native handler, then an error is returned.
    pub fn get_native_handler(&self, fd: i32) -> WasmEdgeResult<u64> {
        let mut handler: u64 = 0;
        let code: u32 = unsafe {
            ffi::WasmEdge_ModuleInstanceWASIGetNativeHandler(
                self.inner.0 as *const _,
                fd,
                &mut handler as *mut u64,
            )
        };

        match code {
            0 => Ok(handler),
            _ => Err(Box::new(WasmEdgeError::Instance(
                InstanceError::NotFoundMappedFdHandler,
            ))),
        }
    }
}
impl AsInstance for CustomWasiModule {
    fn get_func(&self, name: impl AsRef<str>) -> WasmEdgeResult<Function> {
        let func_name: WasmEdgeString = name.as_ref().into();
        let func_ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindFunction(self.inner.0 as *const _, func_name.as_raw())
        };
        match func_ctx.is_null() {
            true => Err(Box::new(WasmEdgeError::Instance(
                InstanceError::NotFoundFunc(name.as_ref().to_string()),
            ))),
            false => Ok(Function {
                inner: InnerFunc(func_ctx),
                registered: true,
            }),
        }
    }

    fn get_table(&self, name: impl AsRef<str>) -> WasmEdgeResult<Table> {
        let table_name: WasmEdgeString = name.as_ref().into();
        let ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindTable(self.inner.0 as *const _, table_name.as_raw())
        };
        match ctx.is_null() {
            true => Err(Box::new(WasmEdgeError::Instance(
                InstanceError::NotFoundTable(name.as_ref().to_string()),
            ))),
            false => Ok(Table {
                inner: InnerTable(ctx),
                registered: true,
            }),
        }
    }

    fn get_memory(&self, name: impl AsRef<str>) -> WasmEdgeResult<Memory> {
        let mem_name: WasmEdgeString = name.as_ref().into();
        let ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindMemory(self.inner.0 as *const _, mem_name.as_raw())
        };
        match ctx.is_null() {
            true => Err(Box::new(WasmEdgeError::Instance(
                InstanceError::NotFoundMem(name.as_ref().to_string()),
            ))),
            false => Ok(Memory {
                inner: InnerMemory(ctx),
                registered: true,
            }),
        }
    }

    fn get_global(&self, name: impl AsRef<str>) -> WasmEdgeResult<Global> {
        let global_name: WasmEdgeString = name.as_ref().into();
        let ctx = unsafe {
            ffi::WasmEdge_ModuleInstanceFindGlobal(self.inner.0 as *const _, global_name.as_raw())
        };
        match ctx.is_null() {
            true => Err(Box::new(WasmEdgeError::Instance(
                InstanceError::NotFoundGlobal(name.as_ref().to_string()),
            ))),
            false => Ok(Global {
                inner: InnerGlobal(ctx),
                registered: true,
            }),
        }
    }

    /// Returns the length of the exported [function instances](crate::Function) in this module instance.
    fn func_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListFunctionLength(self.inner.0) }
    }

    /// Returns the names of the exported [function instances](crate::Function) in this module instance.
    fn func_names(&self) -> Option<Vec<String>> {
        let len_func_names = self.func_len();
        match len_func_names > 0 {
            true => {
                let mut func_names = Vec::with_capacity(len_func_names as usize);
                unsafe {
                    ffi::WasmEdge_ModuleInstanceListFunction(
                        self.inner.0,
                        func_names.as_mut_ptr(),
                        len_func_names,
                    );
                    func_names.set_len(len_func_names as usize);
                }

                let names = func_names
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<String>>();
                Some(names)
            }
            false => None,
        }
    }

    /// Returns the length of the exported [table instances](crate::Table) in this module instance.
    fn table_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListTableLength(self.inner.0) }
    }

    /// Returns the names of the exported [table instances](crate::Table) in this module instance.
    fn table_names(&self) -> Option<Vec<String>> {
        let len_table_names = self.table_len();
        match len_table_names > 0 {
            true => {
                let mut table_names = Vec::with_capacity(len_table_names as usize);
                unsafe {
                    ffi::WasmEdge_ModuleInstanceListTable(
                        self.inner.0,
                        table_names.as_mut_ptr(),
                        len_table_names,
                    );
                    table_names.set_len(len_table_names as usize);
                }

                let names = table_names
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<String>>();
                Some(names)
            }
            false => None,
        }
    }

    /// Returns the length of the exported [memory instances](crate::Memory) in this module instance.
    fn mem_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListMemoryLength(self.inner.0) }
    }

    /// Returns the names of all exported [memory instances](crate::Memory) in this module instance.
    fn mem_names(&self) -> Option<Vec<String>> {
        let len_mem_names = self.mem_len();
        match len_mem_names > 0 {
            true => {
                let mut mem_names = Vec::with_capacity(len_mem_names as usize);
                unsafe {
                    ffi::WasmEdge_ModuleInstanceListMemory(
                        self.inner.0,
                        mem_names.as_mut_ptr(),
                        len_mem_names,
                    );
                    mem_names.set_len(len_mem_names as usize);
                }

                let names = mem_names
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<String>>();
                Some(names)
            }
            false => None,
        }
    }

    /// Returns the length of the exported [global instances](crate::Global) in this module instance.
    fn global_len(&self) -> u32 {
        unsafe { ffi::WasmEdge_ModuleInstanceListGlobalLength(self.inner.0) }
    }

    /// Returns the names of the exported [global instances](crate::Global) in this module instance.
    fn global_names(&self) -> Option<Vec<String>> {
        let len_global_names = self.global_len();
        match len_global_names > 0 {
            true => {
                let mut global_names = Vec::with_capacity(len_global_names as usize);
                unsafe {
                    ffi::WasmEdge_ModuleInstanceListGlobal(
                        self.inner.0,
                        global_names.as_mut_ptr(),
                        len_global_names,
                    );
                    global_names.set_len(len_global_names as usize);
                }

                let names = global_names
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<String>>();
                Some(names)
            }
            false => None,
        }
    }
}
impl AsImport for CustomWasiModule {
    fn name(&self) -> &str {
        "wasi_snapshot_preview1"
    }

    fn add_func(&mut self, name: impl AsRef<str>, mut func: Function) {
        let func_name: WasmEdgeString = name.into();
        unsafe {
            ffi::WasmEdge_ModuleInstanceAddFunction(self.inner.0, func_name.as_raw(), func.inner.0);
        }
        func.inner.0 = std::ptr::null_mut();
    }

    fn add_table(&mut self, name: impl AsRef<str>, mut table: Table) {
        let table_name: WasmEdgeString = name.as_ref().into();
        unsafe {
            ffi::WasmEdge_ModuleInstanceAddTable(self.inner.0, table_name.as_raw(), table.inner.0);
        }
        table.inner.0 = std::ptr::null_mut();
    }

    fn add_memory(&mut self, name: impl AsRef<str>, mut memory: Memory) {
        let mem_name: WasmEdgeString = name.as_ref().into();
        unsafe {
            ffi::WasmEdge_ModuleInstanceAddMemory(self.inner.0, mem_name.as_raw(), memory.inner.0);
        }
        memory.inner.0 = std::ptr::null_mut();
    }

    fn add_global(&mut self, name: impl AsRef<str>, mut global: Global) {
        let global_name: WasmEdgeString = name.as_ref().into();
        unsafe {
            ffi::WasmEdge_ModuleInstanceAddGlobal(
                self.inner.0,
                global_name.as_raw(),
                global.inner.0,
            );
        }
        global.inner.0 = std::ptr::null_mut();
    }
}

// ************** wasi host functions **************

/// `proc_exit` wasi host function
///
/// ```ignore
/// wasi_snapshot_preview1::proc_exit(arg0: i32)
/// ```
///
fn wasi_proc_exit(
    _cf: CallingFrame,
    args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_proc_exit begins");

    let exit_code = args[0].to_i32();
    // let wasi_environ: &mut WasiEnviron = unsafe { &mut *(data as *mut WasiEnviron) };
    let mut wasi_environ = WASI_ENVIRON.write();
    wasi_environ.proc_exit(exit_code);

    println!("<<< wasi_proc_exit ends");
    Ok(vec![])
}

/// fd_write
/// ```ignore
/// wasi_snapshot_preview1::fd_write(
///    fd: i32,
///    iovs: ExternRef,
///    ) -> i32;
/// ```
///
/// In `args`:
/// - `fd`: i32
/// - `iovs`: ExternRef
/// - `nwritten`: i32 (out)
fn wasi_fd_write(cf: CallingFrame, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_fd_write begins");

    let mut memory = cf.memory_mut(0).expect("[wasi_fd_write] memory not found");

    // parse data
    let iovs_offset = args[1].to_i32() as u32;
    let iovs_len = args[2].to_i32() as u32;
    let data = memory
        .get_data(iovs_offset, iovs_len * std::mem::size_of::<Ciovec>() as u32)
        .unwrap();
    let data_ptr = data.as_ptr() as *const Ciovec;
    let iovs = unsafe { std::slice::from_raw_parts(data_ptr, iovs_len as usize) };

    // parse fd
    let fd = args[0].to_i32();

    // write data out
    let mut wasi_environ = WASI_ENVIRON.write();
    let nwritten = wasi_environ.fd_write(fd, iovs);

    // write `nwritten` back to memory
    let nwritten_offset = args[3].to_i32() as u32;
    memory
        .set_data(nwritten.to_le_bytes(), nwritten_offset)
        .expect("[wasi_fd_write] failed to write `nwritten` to memory");

    println!("<<< wasi_fd_write ends");

    Ok(vec![WasmValue::from_i32(0)])
}

/// Returns the number of arguments and the size of the argument string data, or an error.
fn wasi_args_sizes_get(
    cf: CallingFrame,
    args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_args_sizes_get begins");

    let args_size_offset = args[0].to_i32();
    let args_buf_size_offset = args[1].to_i32();

    let wasi_environ = WASI_ENVIRON.read();
    let (n_args, n_bytes) = wasi_environ.args_sizes_get();

    let mut memory = cf
        .memory_mut(0)
        .expect("[wasi_args_sizes_get] memory not found");

    memory
        .set_data(n_args.to_le_bytes(), args_size_offset as u32)
        .expect("[wasi_args_sizes_get] failed to write `n_args` to memory");
    memory
        .set_data(n_bytes.to_le_bytes(), args_buf_size_offset as u32)
        .expect("[wasi_args_sizes_get] failed to write `n_bytes` to memory");

    println!("<<< wasi_args_sizes_get ends");

    Ok(vec![WasmValue::from_i32(0)])
}

/// `args_get` wasi host function
fn wasi_args_get(cf: CallingFrame, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_args_get begins");

    let mut memory = cf
        .memory_mut(0)
        .expect("[wasi_args_sizes_get] memory not found");

    let wasi_environ = WASI_ENVIRON.read();

    let (args_size, args_buf_size) = wasi_environ.args_sizes_get();

    let args_offset = args[0].to_i32();
    let args_buf_offset = args[1].to_i32();

    let mut args_vec = Vec::new();
    wasi_environ.args_get(&mut args_vec);

    let args_mut_ref = memory
        .data_pointer_mut(args_offset as u32, (args_size * 4) as u32)
        .expect("[wasi_args_get] failed to get `args_mut_ref`");
    let mut args_mut_ptr = args_mut_ref as *mut u8;

    let args_buf_mut_ref = memory
        .data_pointer_mut(args_buf_offset as u32, args_buf_size as u32)
        .expect("[wasi_args_get] failed to get `args_buf_mut_ref`");
    let mut args_buf_mut_ptr = args_buf_mut_ref as *mut u8;
    for iov in args_vec.iter() {
        unsafe {
            args_buf_mut_ptr.copy_from(iov.buf, iov.buf_len);
            args_buf_mut_ptr = args_buf_mut_ptr.add(iov.buf_len);

            args_mut_ptr.copy_from(iov.buf_len.to_le_bytes().as_ptr(), 4);
            args_mut_ptr = args_mut_ptr.add(4);
        }
    }

    println!("<<< wasi_args_get ends");

    Ok(vec![WasmValue::from_i32(0)])
}

fn wasi_environ_sizes_get(
    cf: CallingFrame,
    args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_environ_sizes_get begins");

    let environ_size_offset = args[0].to_i32();
    let environ_buf_size_offset = args[1].to_i32();

    let wasi_environ = WASI_ENVIRON.read();
    let (n_envs, n_bytes) = wasi_environ.environ_sizes_get();

    let mut memory = cf
        .memory_mut(0)
        .expect("[wasi_environ_sizes_get] memory not found");

    memory
        .set_data(n_envs.to_le_bytes(), environ_size_offset as u32)
        .expect("[wasi_environ_sizes_get] failed to write `n_envs` to memory");
    memory
        .set_data(n_bytes.to_le_bytes(), environ_buf_size_offset as u32)
        .expect("[wasi_environ_sizes_get] failed to write `n_bytes` to memory");

    println!("<<< wasi_environ_sizes_get ends");

    Ok(vec![WasmValue::from_i32(0)])
}

/// `environ_get` wasi host function
fn wasi_environ_get(
    cf: CallingFrame,
    args: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    println!(">>> wasi_environ_get begins");

    let mut memory = cf
        .memory_mut(0)
        .expect("[wasi_args_sizes_get] memory not found");

    let wasi_environ = WASI_ENVIRON.read();

    let (environ_size, environ_buf_size) = wasi_environ.environ_sizes_get();

    let environ_offset = args[0].to_i32();
    let environ_buf_offset = args[1].to_i32();

    let mut envs_vec = Vec::new();
    wasi_environ.environ_get(&mut envs_vec);

    let environ_mut_ref = memory
        .data_pointer_mut(environ_offset as u32, environ_size as u32 * 4)
        .expect("[wasi_args_get] failed to get `args_ptr`");
    let mut environ_mut_ptr = environ_mut_ref as *mut u8;

    let environ_buf_mut_ref = memory
        .data_pointer_mut(environ_buf_offset as u32, environ_buf_size as u32)
        .expect("[wasi_args_get] failed to get `args_buf_ptr`");
    let mut environ_buf_mut_ptr = environ_buf_mut_ref as *mut u8;
    for iov in envs_vec.iter() {
        unsafe {
            environ_buf_mut_ptr.copy_from(iov.buf, iov.buf_len);
            environ_buf_mut_ptr = environ_buf_mut_ptr.add(iov.buf_len);

            environ_mut_ptr.copy_from(iov.buf_len.to_le_bytes().as_ptr(), 4);
            environ_mut_ptr = environ_mut_ptr.add(4);
        }
    }

    println!("<<< wasi_environ_get ends");

    Ok(vec![WasmValue::from_i32(0)])
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_import_custom_wasi() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, Engine, ImportObject};
    use std::mem::MaybeUninit;
    use wasmedge_wasi_common::{Ciovec, CiovecArray};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `args_sizes_get`
    let fn_args_sizes_get = custom_wasi_module.get_func("args_sizes_get")?;
    let result = vm.run_func(&fn_args_sizes_get, []);
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!((returns[0].to_i32(), returns[1].to_i32()), (2, 10));

    // run `args_get`
    let fn_args_get = custom_wasi_module.get_func("args_get")?;
    let mut iovs: MaybeUninit<Vec<Ciovec>> = MaybeUninit::uninit();
    let result = vm.run_func(&fn_args_get, [WasmValue::from_extern_ref(&mut iovs)]);
    assert!(result.is_ok());
    let iovs = unsafe { iovs.assume_init() };
    // parse the arguments returned
    let mut args_get = vec![];
    for iov in iovs {
        let buf = unsafe { std::slice::from_raw_parts(iov.buf, iov.buf_len) };
        let s = std::str::from_utf8(buf).unwrap();
        args_get.push(s);
    }
    assert_eq!(args_get, ["arg1", "arg2"]);

    // run `environ_sizes_get`
    let fn_environ_sizes_get = custom_wasi_module.get_func("environ_sizes_get")?;
    let result = vm.run_func(&fn_environ_sizes_get, []);
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!((returns[0].to_i32(), returns[1].to_i32()), (3, 30));

    // run `environ_get`
    let mut iovs: MaybeUninit<Vec<Ciovec>> = MaybeUninit::uninit();
    let fn_environ_get = custom_wasi_module.get_func("environ_get")?;
    let result = vm.run_func(&fn_environ_get, [WasmValue::from_extern_ref(&mut iovs)]);
    assert!(result.is_ok());
    let iovs = unsafe { iovs.assume_init() };
    // parse the environment variables returned
    let mut envs_get = vec![];
    for iov in iovs {
        let buf = unsafe { std::slice::from_raw_parts(iov.buf, iov.buf_len) };
        let s = std::str::from_utf8(buf).unwrap();
        envs_get.push(s);
    }
    assert_eq!(envs_get, ["ENV1=VAL1", "ENV2=VAL2", "ENV3=VAL3"]);

    // run `fd_write`
    let fn_fd_write = custom_wasi_module.get_func("fd_write")?;
    let s = "Hello, world!";
    let iov = Ciovec {
        buf: s.as_ptr(),
        buf_len: s.as_bytes().len(),
    };
    let mut iovs: CiovecArray<'_> = &[iov];
    let result = vm.run_func(
        &fn_fd_write,
        [
            WasmValue::from_i32(4),
            WasmValue::from_extern_ref(&mut iovs),
        ],
    );
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!(returns[0].to_i32(), 13);

    // run `proc_exit`
    let fn_proc_exit = custom_wasi_module.get_func("proc_exit")?;
    let _ = vm.run_func(&fn_proc_exit, [WasmValue::from_i32(1)]);

    Ok(())
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_wasi_args_sizes_get() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, AsImport, Engine, ImportObject, MemType, Memory};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let mut import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    // add a custom memory
    let mem_ty = MemType::create(1, None, false)?;
    let memory = Memory::create(&mem_ty)?;
    import_custom_wasi.add_memory("memory", memory);

    // register CustomWasiModule as an import module into vm
    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `args_sizes_get`
    let fn_args_sizes_get = custom_wasi_module.get_func("args_sizes_get")?;
    let result = vm.run_func(
        &fn_args_sizes_get,
        [WasmValue::from_i32(0), WasmValue::from_i32(4)],
    );
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!(returns[0].to_i32(), 0);

    // get the result from the linear memory
    let memory = custom_wasi_module.get_memory("memory")?;
    // get args_size
    let data = memory.get_data(0, 4)?;
    let args_size = i32::from_le_bytes(data.try_into().unwrap());
    assert_eq!(args_size, 2);
    // get args_buf_size
    let data = memory.get_data(4, 4)?;
    let args_buf_size = i32::from_le_bytes(data.try_into().unwrap());
    assert_eq!(args_buf_size, 10);

    Ok(())
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_wasi_environ_sizes_get() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, AsImport, Engine, ImportObject, MemType, Memory};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let mut import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    // add a custom memory
    let mem_ty = MemType::create(1, None, false)?;
    let memory = Memory::create(&mem_ty)?;
    import_custom_wasi.add_memory("memory", memory);

    // register CustomWasiModule as an import module into vm
    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `args_sizes_get`
    let fn_environ_sizes_get = custom_wasi_module.get_func("environ_sizes_get")?;
    let result = vm.run_func(
        &fn_environ_sizes_get,
        [WasmValue::from_i32(0), WasmValue::from_i32(4)],
    );
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!(returns[0].to_i32(), 0);

    // get the result from the linear memory
    let memory = custom_wasi_module.get_memory("memory")?;
    // get environ_size
    let data = memory.get_data(0, 4)?;
    let environ_size = i32::from_le_bytes(data.try_into().unwrap());
    assert_eq!(environ_size, 3);
    // get environ_buf_size
    let data = memory.get_data(4, 4)?;
    let environ_buf_size = i32::from_le_bytes(data.try_into().unwrap());
    assert_eq!(environ_buf_size, 30);

    Ok(())
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_wasi_args_get() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, AsImport, Engine, ImportObject, MemType, Memory};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let mut import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    // add a custom memory
    let mem_ty = MemType::create(1, None, false)?;
    let memory = Memory::create(&mem_ty)?;
    import_custom_wasi.add_memory("memory", memory);

    // register CustomWasiModule as an import module into vm
    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `args_get`
    let fn_args_get = custom_wasi_module.get_func("args_get")?;
    let result = vm.run_func(
        &fn_args_get,
        [WasmValue::from_i32(0), WasmValue::from_i32(8)],
    );
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!(returns[0].to_i32(), 0);

    // get the result from the linear memory
    let memory = custom_wasi_module.get_memory("memory")?;
    // get args_size
    let offset_vec = memory.get_data(0, 8)?;
    let arg1_buf_size = u32::from_le_bytes(offset_vec[0..4].try_into().unwrap());
    dbg!(arg1_buf_size);
    let arg2_buf_size = u32::from_le_bytes(offset_vec[4..8].try_into().unwrap());
    dbg!(arg2_buf_size);
    // get args_buf_size
    let data = memory.get_data(8, arg1_buf_size)?;
    let argument = std::str::from_utf8(&data)?;
    assert_eq!(argument, "arg1");
    let data = memory.get_data(8 + arg1_buf_size, arg2_buf_size)?;
    let argument = std::str::from_utf8(&data)?;
    assert_eq!(argument, "arg2");

    Ok(())
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_wasi_environ_get() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, AsImport, Engine, ImportObject, MemType, Memory};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let envs_len = envs.len();
    let mut import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    // add a custom memory
    let mem_ty = MemType::create(1, None, false)?;
    let memory = Memory::create(&mem_ty)?;
    import_custom_wasi.add_memory("memory", memory);

    // register CustomWasiModule as an import module into vm
    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `args_get`
    let fn_environ_get = custom_wasi_module.get_func("environ_get")?;
    let data_buf_size_offset = 0;
    let data_offset = envs_len as i32 * 4;
    let result = vm.run_func(
        &fn_environ_get,
        [
            WasmValue::from_i32(data_buf_size_offset),
            WasmValue::from_i32(data_offset),
        ],
    );
    assert!(result.is_ok());
    let returns = result.unwrap();
    assert_eq!(returns[0].to_i32(), 0);

    // get the result from the linear memory
    let memory = custom_wasi_module.get_memory("memory")?;
    // get args_size
    let offset_vec = memory.get_data(data_buf_size_offset as u32, data_offset as u32)?;
    let env1_buf_size = u32::from_le_bytes(offset_vec[0..4].try_into().unwrap());
    let env2_buf_size = u32::from_le_bytes(offset_vec[4..8].try_into().unwrap());
    let env3_buf_size = u32::from_le_bytes(offset_vec[8..12].try_into().unwrap());
    // get args_buf_size
    let mut curr_data_offset = data_offset as u32;
    let data = memory.get_data(curr_data_offset, env1_buf_size)?;
    let argument = std::str::from_utf8(&data)?;
    assert_eq!(argument, "ENV1=VAL1");
    curr_data_offset += env1_buf_size;
    let data = memory.get_data(curr_data_offset, env2_buf_size)?;
    let argument = std::str::from_utf8(&data)?;
    assert_eq!(argument, "ENV2=VAL2");
    curr_data_offset += env2_buf_size;
    let data = memory.get_data(curr_data_offset, env3_buf_size)?;
    let argument = std::str::from_utf8(&data)?;
    assert_eq!(argument, "ENV3=VAL3");

    Ok(())
}

#[test]
#[cfg(feature = "custom_wasi")]
fn test_wasi_fd_write() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{vm_new::NewVm, AsImport, Engine, ImportObject, MemType, Memory};

    let mut vm = NewVm::create(None)?;

    // create a CustomWasiModule
    let args = vec!["arg1", "arg2"];
    let envs = vec![("ENV1", "VAL1"), ("ENV2", "VAL2"), ("ENV3", "VAL3")];
    let mut import_custom_wasi = CustomWasiModule::create(Some(args), Some(envs), None)?;

    // add a custom memory
    let mem_ty = MemType::create(1, None, false)?;
    let mut memory = Memory::create(&mem_ty)?;

    // write strings to memory
    let offset = 0;
    let s1 = "Hello, world!".as_bytes();
    memory.set_data(s1, 0)?;
    let s1_ref = memory.data_pointer(offset, s1.len() as u32)?;
    let iov1 = Ciovec {
        buf: s1_ref as *const u8,
        buf_len: s1.len(),
    };
    let s2 = "This is a test.".as_bytes();
    memory.set_data(s2, s1.len() as u32)?;
    let s2_ref = memory.data_pointer(offset + s1.len() as u32, s2.len() as u32)?;
    let iov2 = Ciovec {
        buf: s2_ref as *const u8,
        buf_len: s2.len(),
    };
    let iovs = vec![iov1, iov2];
    let iovs_len = iovs.len();
    let iovs_ptr = iovs.as_ptr() as *const u8;
    let iovs_buf =
        unsafe { std::slice::from_raw_parts(iovs_ptr, std::mem::size_of::<Ciovec>() * iovs.len()) };
    let iovs_offset = offset + s1.len() as u32 + s2.len() as u32;
    memory.set_data(iovs_buf, iovs_offset)?;

    // add memory to CustomWasiModule
    import_custom_wasi.add_memory("memory", memory);

    // register CustomWasiModule as an import module into vm
    vm.register_instance_from_import(ImportObject::CustomWasi(import_custom_wasi))?;

    let custom_wasi_module = vm.custom_wasi_module()?;

    // run `fd_write`
    let fn_fd_write = custom_wasi_module.get_func("fd_write")?;
    let nwritten_offset = iovs_offset + iovs_buf.len() as u32;
    let result = vm.run_func(
        &fn_fd_write,
        [
            WasmValue::from_i32(1), // stdout
            WasmValue::from_i32(iovs_offset as i32),
            WasmValue::from_i32(iovs_len as i32),
            WasmValue::from_i32(nwritten_offset as i32),
        ],
    );
    assert!(result.is_ok());

    // parse the `n_written_bytes` from memory
    let memory = custom_wasi_module.get_memory("memory")?;
    let data = memory.get_data(nwritten_offset, 4)?;
    let n_written_bytes = u32::from_le_bytes(data.try_into().unwrap());
    assert_eq!(n_written_bytes, 28);

    Ok(())
}
