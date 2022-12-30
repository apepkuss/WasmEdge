use crate::error::WasiCommonError;

pub struct StringArray {
    elems: Vec<String>,
}
impl StringArray {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }

    pub fn push(&mut self, elem: String) -> Result<(), WasiCommonError> {
        unimplemented!()
    }
}
