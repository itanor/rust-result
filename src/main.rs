use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Info {
    name: String,
    age: i32,
    rating: i32,
}

#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

fn convenient_methods() {
    let good_result: Result<i32, i32> = Ok(10);
    let bad_result: Result<i32, i32> = Err(10);

    assert!(good_result.is_ok() && !good_result.is_err());
    assert!(bad_result.is_err() && !bad_result.is_ok());

    let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
    let bad_result: Result<i32, i32> = bad_result.map(|i| i - 1);
    println!("good_result: {}", &good_result.unwrap());

    let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));
    println!("good_result: {}", &good_result.unwrap());

    let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
    println!("new bad_result: {}", bad_result.unwrap());
}

fn propagate_error() -> io::Result<()> {
    let mut file = File::create("/tmp/xyz.txt")?;
    file.write_all(b"some message")?;
    Ok(())
}

fn question_mark_operator(info: &Info) -> io::Result<()> {
    let mut file = File::create("/tmp/infos.txt")?;
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}

fn main() {
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
    convenient_methods();
    match propagate_error() {
        Ok(_) => println!("file writed!"),
        Err(e) => println!("error on write file: {:?}", e),
    }
    let info = Info {
        name: String::from("john"),
        age: 34,
        rating: 5,
    };
    question_mark_operator(&info).expect("error on write infos");
}
