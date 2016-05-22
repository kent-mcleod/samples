extern crate nix;

use nix::fcntl;
use std::io::prelude::*;
use std::fs::File;
use std::os::unix::prelude::*;
use std::fs::OpenOptions;

fn main() {
    let f = (File::create("foo.txt")).unwrap();
    let mut file2 = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open("foo2.txt")
                        .unwrap();
    let fd = f.as_raw_fd();
    let res = fcntl::flock(fd, fcntl::FlockArg::LockExclusiveNonblock);
    match res {
        Ok(_) => {
            println!("got lock");
            file2.write_all(b"Hello, world!").expect("Write file:");

            loop {}
        }
        Err(e) => {
            println!("Error: {:?}", e);
            let mut s = String::new();
            file2.read_to_string(&mut s).expect("Read file:");
            println!("File: {}", s);
        }
    }
}
