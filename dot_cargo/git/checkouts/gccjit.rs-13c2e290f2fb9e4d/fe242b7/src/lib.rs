//! # gccjit.rs - Idiomatic Rust bindings to gccjit
//!
//! This library aims to provide idiomatic Rust bindings to gccjit,
//! the embeddable shared library that provides JIT compilation utilizing
//! GCC's backend. See https://gcc.gnu.org/wiki/JIT for more information
//! and for documentation of gccjit itself.
//!
//! Each one of the types provided in this crate corresponds to a pointer
//! type provided by the libgccjit C API. Type conversions are handled by
//! the ToRValue and ToLValue types, which represent values that can be
//! rvalues and values that can be lvalues, respectively.
//!
//! In addition, these types are all statically verified by the Rust compiler to
//! never outlive the Context object from which they came, a requirement
//! to using libgccjit correctly.

extern crate gccjit_sys;

mod asm;
mod types;
mod context;
mod object;
mod location;
mod field;
mod structs;
mod lvalue;
mod rvalue;
mod parameter;
mod function;
mod block;

pub use context::Context;
pub use context::CType;
pub use context::GlobalKind;
pub use context::OptimizationLevel;
pub use context::CompileResult;
pub use context::OutputKind;
pub use location::Location;
pub use object::Object;
pub use object::ToObject;
pub use types::FunctionPtrType;
pub use types::Type;
pub use types::Typeable;
pub use field::Field;
pub use structs::Struct;
#[cfg(feature="master")]
pub use lvalue::{VarAttribute, Visibility};
pub use lvalue::{LValue, TlsModel, ToLValue};
pub use rvalue::{RValue, ToRValue};
pub use parameter::Parameter;
#[cfg(feature="master")]
pub use function::FnAttribute;
pub use function::{Function, FunctionType, InlineMode};
pub use block::{Block, BinaryOp, UnaryOp, ComparisonOp};
