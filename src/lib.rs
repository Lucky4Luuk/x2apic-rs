//! A rust interface to the x2apic interrupt architecture.

#![no_std]
#![feature(ptr_internals)]
#![deny(missing_docs)]

#![feature(const_mut_refs)]
#![feature(const_fn_fn_ptr_basics)]

pub mod ioapic;
pub mod lapic;
