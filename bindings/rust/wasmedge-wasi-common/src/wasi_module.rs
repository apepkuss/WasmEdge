use crate::environ::Environ;

pub struct WasiModule {
    pub name: String,
    pub(crate) ctx: Environ,
}
