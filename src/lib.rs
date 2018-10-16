extern crate libc;
use std::io::prelude::*;

pub fn send(name: String, value: String) {
    let prefix = "/tmp/palombe/";
    let path = format!("{}{}", prefix, name);
    std::fs::create_dir_all(prefix)
        .expect(&format!("Error: couldn't create the {} folder", prefix));
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o644); }
    std::thread::spawn(move || {
        let mut file = std::fs::OpenOptions::new().write(true).open(path)
            .expect("Error: couldn't open the named pipe");
        file.write_all(value.as_bytes())
            .expect("Error: couldn't write the named pipe");
    });
}

pub fn receive(name: String) -> String {
    let prefix = "/tmp/palombe/";
    let path = format!("{}{}", prefix, name);
    let file = std::fs::File::open(path.clone())
        .expect(&format!("Error: couldn't open: {}", path));
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    loop {
        let len = reader.read_line(&mut buffer)
            .expect("Error: couldn't read the input file");
        if len == 0 {
            return buffer;
        }
    }
}