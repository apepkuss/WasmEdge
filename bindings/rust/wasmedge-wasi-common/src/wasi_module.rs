use crate::ctx::WasiCtx;

pub struct WasiModule {
    pub name: String,
    pub(crate) ctx: WasiCtx,
}
