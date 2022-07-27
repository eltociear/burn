use super::{Data, Distribution};
use crate::tensor::Shape;
use std::ops::Range;

pub trait TensorOpsUtilities<P, const D: usize> {
    fn shape(&self) -> &Shape<D>;
    fn into_data(self) -> Data<P, D>;
    fn to_data(&self) -> Data<P, D>;
}

pub trait TensorCreationLike<P, const D: usize> {
    fn new_like_empty(&self) -> Self;
    fn new_like_random(&self, distribution: Distribution<P>) -> Self;
    fn new_like_data(&self, data: Data<P, D>) -> Self;
    fn new_like_zeros(&self) -> Self;
    fn new_like_ones(&self) -> Self;
}

pub trait TensorCreationFork<P, const D: usize, const D2: usize, T: TensorOpsUtilities<P, D2>> {
    fn new_fork_empty(&self, shape: Shape<D2>) -> T;
    fn new_fork_random(&self, shape: Shape<D2>, distribution: Distribution<P>) -> T;
    fn new_fork_data(&self, data: Data<P, D2>) -> T;
    fn new_fork_zeros(&self, shape: Shape<D2>) -> T;
    fn new_fork_ones(&self, shape: Shape<D2>) -> T;
}

pub trait TensorOpsAdd<P, const D: usize>:
    std::ops::Add<Self, Output = Self> + std::ops::Add<P, Output = Self>
where
    Self: Sized,
{
    fn add(&self, other: &Self) -> Self;
    fn add_scalar(&self, other: &P) -> Self;
}

pub trait TensorOpsSub<P, const D: usize>:
    std::ops::Sub<Self, Output = Self> + std::ops::Sub<P, Output = Self>
where
    Self: Sized,
{
    fn sub(&self, other: &Self) -> Self;
    fn sub_scalar(&self, other: &P) -> Self;
}

pub trait TensorOpsTranspose<P, const D: usize> {
    fn transpose(&self) -> Self;
}

pub trait TensorOpsMatmul<P, const D: usize> {
    fn matmul(&self, other: &Self) -> Self;
}

pub trait TensorOpsNeg<P, const D: usize>: std::ops::Neg<Output = Self> {
    fn neg(&self) -> Self;
}

pub trait TensorOpsMul<P, const D: usize>:
    std::ops::Mul<P, Output = Self> + std::ops::Mul<Self, Output = Self>
where
    Self: Sized,
{
    fn mul(&self, other: &Self) -> Self;
    fn mul_scalar(&self, other: &P) -> Self;
}

pub trait TensorOpsReshape<P, const D1: usize, const D2: usize, T: TensorOpsUtilities<P, D2>> {
    fn reshape(&self, shape: Shape<D2>) -> T;
}

pub trait TensorOpsIndex<P, const D1: usize, const D2: usize> {
    fn index(&self, indexes: [Range<usize>; D2]) -> Self;
    fn index_assign(&self, indexes: [Range<usize>; D2], values: &Self) -> Self;
}

pub trait Zeros<T> {
    fn zeros(&self) -> T;
}

pub trait Ones<T> {
    fn ones(&self) -> T;
}
