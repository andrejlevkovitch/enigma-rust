pub mod device;

use crate::device::Device;
use clap::{App, Arg};
use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("Enigma")
        .about("Enigma simulation")
        .arg(
            Arg::with_name("plug-pairs")
                .short('p')
                .long("plug-pairs")
                .help("plug pairs, like \"ABCD\"")
                .default_value(""),
        )
        .arg(
            Arg::with_name("reflector")
                .short('f')
                .long("reflector")
                .help("reflector (A-C)")
                .default_value("B"),
        )
        .arg(
            Arg::with_name("rotor")
                .short('r')
                .long("rotor")
                .value_delimiter(',')
                .help("rotors for usage (I - VIII)")
                .default_values(&["I", "II", "III"]),
        )
        .arg(
            Arg::with_name("segments")
                .short('s')
                .long("segments")
                .value_delimiter(',')
                .help("rotor settings, like: \"ABC\"")
                .default_value(""),
        )
        .arg(
            Arg::with_name("ring-offsets")
                .short('o')
                .long("ring-offsets")
                .help("ring offsets, like \"ABC\"")
                .default_value(""),
        )
        .get_matches();

    let plug_pairs = matches
        .get_one::<String>("plug-pairs")
        .expect("can not be empty");
    let reflector = matches
        .get_one::<String>("reflector")
        .expect("can not be empty");
    let rotors: Vec<String> = matches
        .get_many("rotor")
        .expect("can not be empty")
        .cloned()
        .collect();
    let segments = matches
        .get_one::<String>("segments")
        .expect("can not be empty");
    let ring_offsets = matches
        .get_one::<String>("ring-offsets")
        .expect("can not be empty");

    // create device
    let mut device = Device::new();
    device.set_plug_pairs(plug_pairs.as_str())?;

    if reflector.is_empty() == false {
        device.set_reflector_type(reflector.as_str())?;
    }

    for rotor in rotors.iter() {
        device.add_rotor_type(rotor)?;
    }

    if segments.is_empty() == false {
        device.set_segments(segments.as_str())?;
    }

    if ring_offsets.is_empty() == false {
        device.set_ring_offsets(ring_offsets.as_str())?;
    }

    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                for ch in line.chars() {
                    match device.crypt(ch) {
                        Ok(out) => print!("{}", out),
                        Err(_) => (), // just ignore all errors, like invalid symbol
                    }
                }
            }
            Err(error) => {
                println!("error during reading input: {error}");
                break;
            }
        }
    }

    println!();

    return Ok(());
}
