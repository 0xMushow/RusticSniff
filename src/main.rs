mod frame;
mod packet_parser;

use std::env;
use frame::{Frame, update_frame_id};
use packet_parser::{parse_packet, get_frame_info};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} file.pcap", args[0]);
        std::process::exit(1);
    }

    let file = &args[1];

    let mut frames: Vec<Frame> = Vec::new();

    let builder = rtshark::RTSharkBuilder::builder().input_path(file);

    let mut rtshark = builder.spawn().unwrap_or_else(|e| panic!("Error starting tshark: {e}"));

    while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
        eprintln!("Error parsing tshark output: {e}");
        None
    }) {
        println!("--- NEW FRAME ---");
        println!("{}", frames.len());

        let combined_string: String = packet
            .iter()
            .flat_map(|layer| layer.iter().map(|data| format!("\n\t{}", data.display())))
            .collect();

        println!("{}", combined_string);

        let metadata = parse_packet(&packet);
        let mut frame = get_frame_info(metadata);

        update_frame_id(&mut frame, &frames);
        frames.push(frame);
    }
}