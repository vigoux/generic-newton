# newton

A dead-simple, dependency-free Rust crate for generic Newton method.

## Usage

Everything is contained in the `Newton` iterator :

```
use newton::Newton;

fn main() {
     let mut n = Newton::<f64>::new(
         0.5, // Initial guess
         |x| x.cos() - x.powi(3), // The actual function
         |x| -(x.sin() + 3. * x.powi(2)), // Its derivative
     );

     assert!(n.nth(1000).unwrap() - 0.865474033102 < 1E-11)
}
```
