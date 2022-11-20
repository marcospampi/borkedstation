#![feature(pointer_byte_offsets)]
#![feature(exclusive_range_pattern)]
pub mod cpu;
mod machine;
pub mod bus;
pub use machine::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    
    }
}
