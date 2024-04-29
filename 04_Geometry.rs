// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.


fn magnitude(v: &[f64]) -> f64 {
    let mut square_sum = 0f64; // f64 with value 0
    
    // Sum of squares
    for i in v {
        square_sum += i * i;
    }
    
    // square root of sum of squares
    square_sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(v: &mut [f64]) {

    // calculate magnitude before to avoid borrow after move
    let mag = magnitude(v); // Already a ref so no need of &
    
    for i in v {
        // Each element is a &f64 so need to deref and assign
        *i /= mag;
    }
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
