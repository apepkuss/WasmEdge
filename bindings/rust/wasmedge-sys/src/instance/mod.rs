//! Defines WasmEdge instance structs, including Function, Global, Memory, and Table.

#[cfg(feature = "custom_wasi")]
pub mod custom_wasi_module;
pub mod function;
pub mod global;
pub mod memory;
pub mod module;
pub mod table;

#[doc(hidden)]
pub use function::{FuncType, Function};
#[doc(hidden)]
pub use global::{Global, GlobalType};
#[doc(hidden)]
pub use memory::{MemType, Memory};
#[doc(hidden)]
pub use module::Instance;
#[doc(hidden)]
pub use table::{Table, TableType};
