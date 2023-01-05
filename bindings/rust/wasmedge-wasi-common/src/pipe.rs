use crate::file::WasiFile;
use std::{
    any::Any,
    io::{self, Read, Write},
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct ReadPipe<R: Read> {
    reader: Arc<RwLock<R>>,
}
impl<R: Read> ReadPipe<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: Arc::new(RwLock::new(reader)),
        }
    }
}
impl<R: Read + Any + Send + Sync> WasiFile for ReadPipe<R> {}

#[derive(Debug)]
pub struct WritePipe<W: Write> {
    writer: Arc<RwLock<W>>,
}
impl<W: Write> WritePipe<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: Arc::new(RwLock::new(writer)),
        }
    }
}
impl<W: Write + Any + Send + Sync> WasiFile for WritePipe<W> {}
