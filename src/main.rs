mod sniffer;
extern crate nix;
use std::env;
use sniffer::get_heap_range;
use nix::unistd::getuid;



fn main() -> Result<(), ()> {
    if !getuid().is_root() {
        println!("You must be running as root!");
        return Err(());
    }

    let arguments: Vec<String> = env::args().collect();

    for arg in 1..arguments.len() { 
        match arguments[arg].parse::<u32>() {
            Ok(ok) => {
                let (heap_start, heap_end) = get_heap_range(ok)?;
                println!("Program heap starts at {} and ends at {}.", heap_start, heap_end);
            }

            Err(err) => {
                println!("Invalid argument \"{}\"!: {}", arguments[arg], err);
                break;
            }
        }
    } 

    Ok(())
}
