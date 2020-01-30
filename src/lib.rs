#![allow(incomplete_features)]
#![feature(const_generics,fn_traits,unboxed_closures,trait_alias,const_compare_raw_pointers,option_unwrap_none,new_uninit,box_syntax,non_ascii_idents)]
pub use std::{cmp::max};
pub trait Add<T=Self> = std::ops::Add<T,Output=T>;
pub trait Sub<T=Self> = std::ops::Sub<T,Output=T>;
pub trait Mul<T> = std::ops::Mul<T,Output=T>;
pub use framework::{core::{Zero,Result,mask,abs,cb,sqrt}, vector::{xy,uint2,int2,vec2}};
mod compose;
mod algebra; pub use algebra::{Idx,collect,Sum};
#[macro_use] pub mod mesh; pub use mesh::{Mesh,mesh,Field,Matrix,operator,Equation,I,P,Δ,Dx,Dy,border};
#[allow(non_snake_case)] pub mod SI;
#[macro_use] pub mod view; pub use {framework::window::run, view::{Solution,View}};
/*#[macro_export] macro_rules! prelude { {} => {
#[allow(incomplete_features)]
#![feature(const_generics)]
use simulation::*;
} }*/
