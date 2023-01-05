use crate::{
    dir::{DirCaps, DirEntry, WasiDir},
    error::WasiCommonError,
    file::{FileCaps, FileEntry, WasiFile},
    pipe::{ReadPipe, WritePipe},
    FdMap, StringArray,
};

pub struct Environ {
    pub args: StringArray,
    pub envs: StringArray,
    pub fd_map: FdMap,
}
impl Environ {
    pub fn new() -> Self {
        let mut environ = Self {
            args: StringArray::new(),
            envs: StringArray::new(),
            fd_map: FdMap::new(),
        };
        environ.set_stdin(Box::new(ReadPipe::new(std::io::empty())));
        environ.set_stdout(Box::new(WritePipe::new(std::io::sink())));
        environ.set_stderr(Box::new(WritePipe::new(std::io::sink())));
        environ
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

    pub fn insert_file(&mut self, fd: i32, file: Box<dyn WasiFile>, caps: FileCaps) {
        self.fd_map
            .insert_at(fd, Box::new(FileEntry::new(caps, file)));
    }

    pub fn set_stdin(&mut self, mut file: Box<dyn WasiFile>) {
        let caps = Self::stdio_caps(&mut *file);
        self.insert_file(0, file, caps);
    }

    pub fn set_stdout(&mut self, mut file: Box<dyn WasiFile>) {
        let caps = Self::stdio_caps(&mut *file);
        self.insert_file(1, file, caps);
    }

    pub fn set_stderr(&mut self, mut file: Box<dyn WasiFile>) {
        let caps = Self::stdio_caps(&mut *file);
        self.insert_file(2, file, caps);
    }

    fn stdio_caps(file: &mut dyn WasiFile) -> FileCaps {
        let mut caps = FileCaps::all();

        // If `file` is a tty, restrict the `tell` and `seek` capabilities, so
        // that wasi-libc's `isatty` correctly detects the file descriptor
        // as a tty.
        if file.is_tty() {
            caps &= !(FileCaps::TELL | FileCaps::SEEK);
        }

        caps
    }
}
