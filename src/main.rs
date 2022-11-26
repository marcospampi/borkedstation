#![feature(pointer_byte_offsets)]
#![feature(exclusive_range_pattern)]
mod core;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {

    }
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use crate::core::machine::Machine;

    #[test]
    fn test_boot_bios() {
        let bios = std::env::var("PSX_BIOS").unwrap();

        let machine = Machine::new_with_bios(&bios).unwrap();
        machine.run()
    }
}
