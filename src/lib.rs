#![feature(assert_matches)]
#![feature(box_patterns)]
#![feature(generic_arg_infer)]
#![feature(impl_trait_in_bindings)]
#![feature(let_chains)]

pub mod backend;
pub mod cli;
pub mod frontend;
pub mod inference;
pub mod ir;
pub mod tpch;
