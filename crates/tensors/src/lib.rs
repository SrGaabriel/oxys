use std::marker::PhantomData;

use thiserror::Error;

pub trait Location {
    fn vadd();
}

pub struct Tensor<L: Location, T, const D: usize> {
    data: Vec<T>,
    shape: [usize; D],
    stride: [usize; D],
    _p: PhantomData<L>,
}

pub type Vector<L, T> = Tensor<L, T, 1>;
pub type Matrix<L, T> = Tensor<L, T, 2>;

pub trait TensorLike {
    fn shape(&self) -> &[usize];
    fn stride(&self) -> &[usize];
    fn numel(&self) -> usize;
    fn rank(&self) -> usize {
        self.shape().len()
    }
    fn is_contiguous(&self) -> bool;
    fn is_empty(&self) -> bool {
        self.numel() == 0
    }
}

#[derive(Debug, Error)]
pub enum TensorError {
    #[error("shape mismatch: expected {expected} elements from shape, got {got}")]
    ShapeMismatch { expected: usize, got: usize },
}

impl<L: Location, T, const D: usize> Tensor<L, T, D> {
    pub fn new(shape: [usize; D], data: Vec<T>) -> Result<Self, TensorError> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(TensorError::ShapeMismatch {
                expected,
                got: data.len(),
            });
        }
        let stride = Self::compute_stride(&shape);
        Ok(Self {
            data,
            shape,
            stride,
            _p: PhantomData,
        })
    }

    pub fn from_shape_fn<F>(shape: [usize; D], mut f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let numel = shape.iter().product();
        let data = (0..numel).map(|_| f()).collect();
        Self {
            data,
            shape,
            stride: Self::compute_stride(&shape),
            _p: PhantomData,
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    fn compute_stride(shape: &[usize; D]) -> [usize; D] {
        let mut stride = [0; D];
        if D > 0 {
            stride[D - 1] = 1;

            for i in (0..D - 1).rev() {
                stride[i] = stride[i + 1] * shape[i + 1];
            }
        }
        stride
    }
}

impl<L: Location, T, const D: usize> TensorLike for Tensor<L, T, D> {
    fn shape(&self) -> &[usize] {
        &self.shape
    }
    fn numel(&self) -> usize {
        self.data.len()
    }

    fn stride(&self) -> &[usize] {
        &self.stride
    }
    fn is_contiguous(&self) -> bool {
        Self::compute_stride(&self.shape) == self.stride
    }
}

impl<L: Location, T> Matrix<L, T> {
    pub fn rows(&self) -> usize {
        self.shape[0]
    }
    pub fn cols(&self) -> usize {
        self.shape[1]
    }
}
