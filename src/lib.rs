//! A lightweight implementation of verlet 2d physics engine in
//! pure rust
//! 
//! # Example
//! 
//! ```ignore
//! ```

// forbid unsafe code
#![forbid(unsafe_code)]

pub mod behaviors;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
