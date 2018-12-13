use super::*;

/// The diagonal of an N-dimensional homotopy, resulting in a 1D homotopy.
///
/// This interpolates along all dimensions at once.
#[derive(Copy, Clone)]
pub struct Diagonal<T, S> {
    shape: T,
    _s: PhantomData<S>,
}

impl<T, S> Diagonal<T, S> {
    /// Creates a new diagonal.
    pub fn new(shape: T) -> Self {
        Diagonal {shape, _s: PhantomData}
    }
}

impl<X, T> Homotopy<X> for Diagonal<T, [f64; 2]>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.shape.f(x)}
    fn g(&self, x: X) -> Self::Y {self.shape.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.shape.h(x, [s; 2])}
}

impl<X, T> Homotopy<X> for Diagonal<T, [f64; 3]>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.shape.f(x)}
    fn g(&self, x: X) -> Self::Y {self.shape.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.shape.h(x, [s; 3])}
}

/// The left side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Left<T>(pub T);

impl<X, T> Homotopy<X> for Left<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [0.0, s])}
}

impl<X, T> Homotopy<X, [f64; 2]> for Left<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [0.0, s[0], s[1]])}
}

/// The right side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Right<T>(pub T);

impl<X, T> Homotopy<X> for Right<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [1.0, s])}
}

impl<X, T> Homotopy<X, [f64; 2]> for Right<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [1.0, s[0], s[1]])}
}

/// The top side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Top<T>(pub T);

impl<X, T> Homotopy<X> for Top<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [s, 0.0])}
}

impl<X, T> Homotopy<X, [f64; 2]> for Top<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 0.0, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], 0.0, s[1]])}
}

/// The bottom side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Bottom<T>(pub T);

impl<X, T> Homotopy<X> for Bottom<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [s, 1.0])}
}

impl<X, T> Homotopy<X, [f64; 2]> for Bottom<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 1.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], 1.0, s[1]])}
}

/// The front side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Front<T>(pub T);

impl<X, T> Homotopy<X, [f64; 2]> for Front<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.f(x)}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 1.0, 0.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], s[1], 0.0])}
}

/// The back side of an N-dimensional homotopy, resulting in a N-1 homotopy.
#[derive(Copy, Clone)]
pub struct Back<T>(pub T);

impl<X, T> Homotopy<X, [f64; 2]> for Back<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 0.0, 1.0])}
    fn g(&self, x: X) -> Self::Y {self.0.g(x)}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], s[1], 1.0])}
}

/// Intersects from left to right.
#[derive(Copy, Clone)]
pub struct LeftRight<T>(pub T, pub f64);

impl<X, T> Homotopy<X> for LeftRight<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [self.1, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [self.1, 1.0])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [self.1, s])}
}

impl<X, T> Homotopy<X, [f64; 2]> for LeftRight<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [self.1, 0.0, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [self.1, 1.0, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [self.1, s[0], s[1]])}
}

/// Intersects from top to botttom.
#[derive(Copy, Clone)]
pub struct TopBottom<T>(pub T, pub f64);

impl<X, T> Homotopy<X> for TopBottom<T>
    where T: Homotopy<X, [f64; 2]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, self.1])}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, self.1])}
    fn h(&self, x: X, s: f64) -> Self::Y {self.0.h(x, [s, self.1])}
}

impl<X, T> Homotopy<X, [f64; 2]> for TopBottom<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, self.1, 0.0])}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, self.1, 1.0])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], self.1, s[1]])}
}

/// Intersects from front to back.
#[derive(Copy, Clone)]
pub struct FrontBack<T>(pub T, pub f64);

impl<X, T> Homotopy<X, [f64; 2]> for FrontBack<T>
    where T: Homotopy<X, [f64; 3]>
{
    type Y = T::Y;

    fn f(&self, x: X) -> Self::Y {self.0.h(x, [0.0, 0.0, self.1])}
    fn g(&self, x: X) -> Self::Y {self.0.h(x, [1.0, 1.0, self.1])}
    fn h(&self, x: X, s: [f64; 2]) -> Self::Y {self.0.h(x, [s[0], s[1], self.1])}
}
