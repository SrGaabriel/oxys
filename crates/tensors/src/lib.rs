#![feature(adt_const_params)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(Debug, PartialEq, Eq, ConstParamTy)]
pub enum Location {
    Host,
    Device
}

pub struct Tensor<const L: Location, const M: usize, const N: usize> {
    
}
