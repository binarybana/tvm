
use crate::{DataType};
use crate::runtime::{Object};

pub mod array;
pub mod relay;

#[repr(C)]
pub struct PrimExprNode {
    pub base: Object,
    pub dtype: DataType,
}

#[repr(C)]
pub struct IntImmNode {
    pub base: PrimExprNode,
    pub value: i64,
}
