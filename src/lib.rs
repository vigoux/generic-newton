use std::iter::Iterator;
use std::ops::{Div, Sub};

/// An iterator that returns successive iterations of the Newton's method.
///
/// This iterator is generic over it's entry, and allows to freely use the Newton method on
/// arbitrary types.
///
/// # Example
///
/// ```
/// use generic_newton::Newton;
///
/// fn main() {
///     let mut n = Newton::<f64,_,_>::new(
///         0.5, // Initial guess
///         |x| x.cos() - x.powi(3), // The actual function
///         |x| -(x.sin() + 3. * x.powi(2)), // Its derivative
///     );
///
///     assert!((n.nth(1000).unwrap() - 0.865474033102).abs() < 1E-11)
/// }
/// ```
pub struct Newton<T, F, DF>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
    F: Fn(T) -> T,
    DF: Fn(T) -> T,
{
    current: T,
    func: F,
    derivative: DF,
}

impl<T, F, DF> Newton<T, F, DF>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
    F: Fn(T) -> T,
    DF: Fn(T) -> T,
{
    /// Creates a new `Newton` iterator.
    ///
    /// - `func` is the actual function to find the root of
    /// - `derivative` is it's derivative.
    pub fn new(initial_guess: T, func: F, derivative: DF) -> Self {
        Newton {
            current: initial_guess,
            func,
            derivative,
        }
    }
}

impl<T, F, DF> Iterator for Newton<T, F, DF>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
    F: Fn(T) -> T,
    DF: Fn(T) -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let func = &self.func;
        let deriv = &self.derivative;

        let next = self.current - (func(self.current) / deriv(self.current));

        self.current = next;
        Some(next)
    }
}

#[cfg(test)]
mod tests {

    use super::Newton;

    #[test]
    fn is_generic() {
        let mut n = Newton::new(0, |x| x - 1, |_| 1);
        assert_eq!(n.nth(5).unwrap(), 1);

        let mut n = Newton::new(0., |x| x - 1., |_| 1.);
        assert_eq!(n.nth(5).unwrap(), 1.);

        let value = 1.0;
        let mut n = Newton::new(0., |x| x - value, |_| 1.);
        assert_eq!(n.nth(5).unwrap(), 1.);
    }
}
