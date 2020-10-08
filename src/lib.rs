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
///     let mut n = Newton::<f64>::new(
///         0.5, // Initial guess
///         |x| x.cos() - x.powi(3), // The actual function
///         |x| -(x.sin() + 3. * x.powi(2)), // Its derivative
///     );
///
///     assert!((n.nth(1000).unwrap() - 0.865474033102).abs() < 1E-11)
/// }
/// ```
pub struct Newton<T>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
{
    current: T,
    func: fn(T) -> T,
    derivative: fn(T) -> T,
}

impl<T> Newton<T>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
{
    /// Creates a new `Newton` iterator.
    ///
    /// - `func` is the actual function to find the root of
    /// - `derivative` is it's derivative.
    pub fn new(initial_guess: T, func: fn(T) -> T, derivative: fn(T) -> T) -> Self {
        Newton {
            current: initial_guess,
            func,
            derivative,
        }
    }
}

impl<T> Iterator for Newton<T>
where
    T: Div<Output = T> + Sub<Output = T> + Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let func = self.func;
        let deriv = self.derivative;

        let next = self.current - (func(self.current) / deriv(self.current));
        let prev = self.current;

        self.current = next;

        Some(prev)
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
    }
}
