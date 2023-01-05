use crate::{
    dir::{DirCaps, DirEntry, WasiDir},
    error::WasiCommonError,
    file::{FileCaps, FileEntry, WasiFile},
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

    pub fn push_file(
        &mut self,
        file: Box<dyn WasiFile>,
        caps: FileCaps,
    ) -> Result<i32, WasiCommonError> {
        self.fd_map.push(Box::new(FileEntry::new(caps, file)))
    }

    pub fn insert_file(
        &mut self,
        fd: i32,
        file: Box<dyn WasiFile>,
        caps: FileCaps,
    ) -> Result<(), WasiCommonError> {
        Ok(self
            .fd_map
            .insert_at(fd, Box::new(FileEntry::new(caps, file))))
    }

    pub fn set_stdin(&mut self, mut file: Box<dyn WasiFile>) {
        unimplemented!()
    }

    pub fn set_stdout(&mut self, mut file: Box<dyn WasiFile>) {
        unimplemented!()
    }

    pub fn set_stderr(&mut self, mut file: Box<dyn WasiFile>) {
        unimplemented!()
    }

    fn stdio_caps(file: &mut dyn WasiFile) -> FileCaps {
        let mut caps = FileCaps::all();

        if file.isatty() {
            caps &= !(FileCaps::TELL | FileCaps::SEEK);
        }

        caps
    }
}
