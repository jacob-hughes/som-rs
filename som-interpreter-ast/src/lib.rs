//!
//! This is the interpreter for the Simple Object Machine.
//!

#![feature(gc)]
use std::gc::GcAllocator;

#[global_allocator]
static A: GcAllocator = GcAllocator;

use std::cell::RefCell;
use std::gc::Gc;

/// Facilities for manipulating blocks.
pub mod block;
/// Facilities for manipulating classes.
pub mod class;
/// Facilities for evaluating nodes and expressions.
pub mod evaluate;
/// Facilities for manipulating stack frames.
pub mod frame;
/// Facilities for manipulating values.
pub mod hashcode;
/// Facilities for manipulating class instances.
pub mod instance;
/// Facilities for string interning.
pub mod interner;
/// Facilities for invoking methods and/or primitives.
pub mod invokable;
/// Facilities for manipulating class methods.
pub mod method;
/// Definitions for all supported primitives.
pub mod primitives;
/// The interpreter's main data structure.
pub mod universe;
/// Facilities for manipulating values.
pub mod value;

/// A strong and owning reference to an object.
pub type SOMRef<T> = Gc<RefCell<T>>;
