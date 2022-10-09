pub mod device;

use crate::device::rotor::Rotor;
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
                .help("plug pairs")
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
                .help("rotors for usage (I - VIII)")
                .default_values(&["I", "II", "III"]),
        )
        .arg(
            Arg::with_name("segments")
                .short('s')
                .long("segments")
                .help("rotor settings")
                .default_value(""),
        )
        .arg(
            Arg::with_name("ring-offsets")
                .short('o')
                .long("ring-offsets")
                .help("ring offsets")
                .default_value(""),
        )
        .get_matches();

    let plug_pairs: String = matches.value_of_t_or_exit("plug-pairs");
    let reflector: char = matches.value_of_t_or_exit("reflector");
    let mut rotors: Vec<String> = matches.values_of_t_or_exit("rotor");
    let segments: String = matches.value_of_t_or_exit("segments");
    let ring_offsets: String = matches.value_of_t_or_exit("ring-offsets");

    // create device
    let mut device = Device::new();
    device.set_plug_pairs(plug_pairs.as_str())?;

    match reflector.to_ascii_uppercase() {
        'A' => device.set_reflector(Rotor::reflector_a()),
        'B' => device.set_reflector(Rotor::reflector_b()),
        'C' => device.set_reflector(Rotor::reflector_c()),
        _ => panic!("unknown reflector type: {}", reflector),
    }

    for rotor in rotors.iter_mut() {
        rotor.make_ascii_uppercase();

        if rotor == "I" {
            device.add_rotor(Rotor::rotor_i());
        } else if rotor == "II" {
            device.add_rotor(Rotor::rotor_ii());
        } else if rotor == "III" {
            device.add_rotor(Rotor::rotor_iii());
        } else if rotor == "IV" {
            device.add_rotor(Rotor::rotor_iv());
        } else if rotor == "V" {
            device.add_rotor(Rotor::rotor_v());
        } else if rotor == "VI" {
            device.add_rotor(Rotor::rotor_vi());
        } else if rotor == "VII" {
            device.add_rotor(Rotor::rotor_vii());
        } else if rotor == "VIII" {
            device.add_rotor(Rotor::rotor_viii());
        } else {
            panic!("unknown rotor type: {}", rotor);
        }
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
