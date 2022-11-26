#![feature(pointer_byte_offsets)]
mod core;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {

    }
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    #[test]
    fn test_run() {
        //let bios = std::env::var("PSX_BIOS").unwrap();
//
        //let machine = Machine::new(&bios).unwrap();
//
        ////if let Ok(mut cpu) =  machine.cpu.try_borrow_mut() {
        ////    cpu.
        ////};
        //machine.run();
    }
}
