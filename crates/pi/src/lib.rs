pub mod config;
pub mod error;
pub mod pkg;

pub use error::PiError;
pub use pkg::Pkg;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
