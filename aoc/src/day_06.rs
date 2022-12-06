use std::fs;
use itertools::Itertools;

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/06_input_00.txt")
        .expect("[-] Couldn't read the file");
    let marker_len = 4;
    let msg_marker_len = 14;
    let stream_p1 = input.as_bytes().windows(marker_len);
    let stream_p2 = input.as_bytes().windows(msg_marker_len);
    let mut packet_start = 0;
    let mut msg_start = 0;
    let mut index = 0;

    // part 1
    for i in stream_p1 {
        let window: Vec<&u8> = i.iter().unique().collect();

        // all are unique, we found the packet start
        if window.len() == marker_len {
            packet_start = index + marker_len;
            break;
        }
        
        index += 1;
    }

    // part 2
    index = 0;
    for i in stream_p2 {
        let window: Vec<&u8> = i.iter().unique().collect();

        // all are unique, we found the msg start
        if window.len() == msg_marker_len {
            msg_start = index + msg_marker_len;
            break;
        }
        
        index += 1;
    }

    println!("[06] Solution a: {}", packet_start);
    println!("[06] Solution b: {}", msg_start);
}
