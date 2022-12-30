use crate::{
    dir::{DirCaps, DirEntry, WasiDir},
    error::WasiCommonError,
    file::FileCaps,
    FdMap, StringArray,
};

pub struct WasiCtx {
    pub args: StringArray,
    pub envs: StringArray,
    pub fd_map: FdMap,
}
impl WasiCtx {
    pub fn new() -> Self {
        Self {
            args: StringArray::new(),
            envs: StringArray::new(),
            fd_map: FdMap::new(),
        }
    }

    pub fn push_arg(&mut self, arg: impl AsRef<str>) -> Result<(), WasiCommonError> {
        self.args.push(arg.as_ref().to_owned())
    }

    pub fn push_env(
        &mut self,
        var: impl AsRef<str>,
        val: impl AsRef<str>,
    ) -> Result<(), WasiCommonError> {
        self.envs.push(format!("{}={}", var.as_ref(), val.as_ref()))
    }

    pub fn push_dir(
        &mut self,
        dir: Box<dyn WasiDir>,
        caps: DirCaps,
        file_caps: FileCaps,
        preopen_dir: std::path::PathBuf,
    ) -> Result<i32, WasiCommonError> {
        self.fd_map.push(Box::new(DirEntry::new(
            caps,
            file_caps,
            Some(preopen_dir),
            dir,
        )))
    }
}
