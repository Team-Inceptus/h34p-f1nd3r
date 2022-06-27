use std::fs;
use regex::Regex;

pub fn get_heap_range(pid: u32) -> Result<(String, String), ()> {
    let maps_filename = format!("/proc/{}/maps", pid);
    let maps: String = fs::read_to_string(maps_filename).unwrap();
    let heap = Regex::new(".*\\[heap\\]").unwrap();

    if !heap.is_match(maps.as_str()) {
        println!("CAN'T FIND HEAP IN THIS PROCCESS!\n");
        return Err(());
    }

    // Heap start & heap end.
    let mut start_addr: String = String::new();
    let mut end_addr: String = String::new();

    for cap in heap.captures_iter(maps.as_str()) {
        let line: &str = cap.get(0).unwrap().as_str();
        let mut get_start_addr: bool = true;

        for byte in line.chars() {
            if get_start_addr && byte == '-' {
                get_start_addr = false;
                continue;
            } else if !get_start_addr && byte == ' ' {
                break;
            }

            if get_start_addr {
                start_addr.push(byte);
            } else {
                end_addr.push(byte);
            }
        }
    }

    return Ok((start_addr, end_addr))
}
