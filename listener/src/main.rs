extern crate argparse;
extern crate protobuf;
extern crate serial;

use argparse::{ArgumentParser, Store};
use protobuf::parse_from_reader;
use serial::prelude::*;
use std::io::Read;

pub mod telemetry;

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

    let parsed = parse_from_reader::<telemetry::Telemetry>(&mut port);
}
