extern crate argparse;
extern crate serial;

use argparse::{ArgumentParser, Store};
use serial::prelude::*;
use std::io::Read;

fn main() {
    // Gather configuration.
    let mut device = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Listen to lighting signals.");
        ap.refer(&mut device)
            .add_option(&["--device"], Store, "The device name");
        ap.parse_args_or_exit();
    }

    // Configure the device to listen on.
    let mut port = serial::open(&device).unwrap();
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        Ok(())
    }).unwrap();

    // Listen to incoming data.
    let header = b'~';
    let mut header_buffer: [u8; 1] = [0];
    let mut packet_buffer: [u8; 3] = [0, 0, 0];
    let mut movement: u8;
    let mut luminosity: u16;
    loop {
        if let Err(_) = port.read_exact(&mut header_buffer) {
            // Ignore errors for now.
            continue;
        }
        if header_buffer[0] != header {
            // Skip unexpected bytes.
            continue;
        }
        if let Err(_) = port.read_exact(&mut packet_buffer) {
            // Ignore errors for now.
            continue;
        }
        movement = packet_buffer[0];
        luminosity = packet_buffer[1] as u16 + ((packet_buffer[2] as u16) << 8);
        println!("PACKET:");
        println!("Movement: {:?}", movement);
        println!("Luminosity: {:?}", luminosity);
    }
}
