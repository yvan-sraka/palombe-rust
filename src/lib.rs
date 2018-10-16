#![feature(specialization)]
extern crate libc;
use std::io::prelude::*;

#[macro_use]
extern crate pyo3;
use pyo3::prelude::*;

fn __mkfifo(name: String) -> String {
    let prefix = "/tmp/palombe/";
    let path = format!("{}{}", prefix, name);
    std::fs::create_dir_all(prefix)
        .expect(&format!("Error: couldn't create the folder {}", prefix));
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o600); }
    return path;
}

#[pyfunction]
#[no_mangle]
pub extern "C" fn send(name: String, value: String) {
    let path = __mkfifo(name);
    let mut file = std::fs::OpenOptions::new().write(true).open(path)
        .expect("Error: couldn't open the named pipe");
    file.write_all(value.as_bytes())
        .expect("Error: couldn't write the named pipe");
}

#[pyfunction]
#[no_mangle]
pub extern "C" fn receive(name: String) -> String {
    let path = __mkfifo(name);
    let file = std::fs::File::open(path.clone())
        .expect(&format!("Error: couldn't open: {}", path));
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    loop {
        let len = reader.read_line(&mut buffer)
            .expect("Error: couldn't read the input file");
        if len == 0 {
            std::fs::remove_file(&path)
                .expect(&format!("Error: couldn't remove the file {}", path));
            return buffer;
        }
    }
}