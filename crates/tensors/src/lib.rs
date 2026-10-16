//#![cfg_attr(feature = "nightly", allow(incomplete_features))]

// use std::marker::ConstParamTy;
// #[cfg_attr(feature = "nightly", derive(ConstParamTy))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Host,
    Device,
}

//#[cfg_attr(predicate, attr)]
pub struct Tensor<T, const D: usize> {
    data: Vec<T>,
    shape: [usize; D],
    stride: [usize; D],
    location: Location,
} 

pub type Vector<T> = Tensor<T, 1>;
pub type Matrix<T> = Tensor<T, 2>;


pub trait TensorLike{
    fn shape(&self) -> &[usize];
    fn stride(&self) -> &[usize];
    fn numel(&self) -> usize;
    fn rank(&self) -> usize {
        self.shape().len()
    }
    fn is_contiguous(&self) -> bool;
    fn location(&self) -> Location;
    fn is_empty(&self) -> bool {
        self.numel() == 0
    }
}


#[derive(Debug)]
pub struct ShapeMismatch {
    expected: usize,
    got: usize
}

impl std::fmt::Display for ShapeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shape mismatch: expected {} elements from shape, got {}", self.expected, self.got)
    }
}

impl std::error::Error for ShapeMismatch {}

impl<T, const D: usize> Tensor<T, D> {
    pub fn new(shape: [usize; D], data: Vec<T>) -> Result<Self, ShapeMismatch> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(ShapeMismatch { expected, got: data.len() });
        }
        let stride = Self::compute_stride(&shape);
        Ok(Self {
            data,
            shape,
            stride,
            location: Location::Host
        })
    }

    pub fn new_with_location(shape: [usize; D], data: Vec<T>, location: Location) -> Result<Self, ShapeMismatch> {
        let mut t = Self::new(shape, data)?;
        t.location = location;
        Ok(t)
    }

    pub fn from_shape_fn<F> (shape: [usize; D], mut f: F) -> Self where F:FnMut() -> T {
        let numel = shape.iter().product();
        let data = (0..numel).map(|_| f()).collect();
        Self {
            data, 
            shape,
            stride: Self::compute_stride(&shape),
            location: Location::Host
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
            stride[D-1] = 1;
        
            for i in (0..D-1).rev() {
                stride[i] = stride[i + 1] * shape[i + 1];
            }
        }
        stride
    }
}


impl<T, const D: usize> TensorLike for Tensor<T, D> {
    fn shape(&self) -> &[usize] {
        &self.shape
    }
    fn numel(&self) -> usize {
        self.data.len()
    }
    fn location(&self) -> Location {
        self.location
    }
    fn stride(&self) -> &[usize] {
        &self.stride
    }
    fn is_contiguous(&self) -> bool {
        Self::compute_stride(&self.shape) == self.stride
    }
}

impl<T> Vector <T> {
    pub fn len(&self) -> usize {
        self.shape[0]
    }
}

impl<T> Matrix <T> {
    pub fn rows(&self) -> usize {
        self.shape[0]
    }
    pub fn cols(&self) -> usize {
        self.shape[1]
    }
}
