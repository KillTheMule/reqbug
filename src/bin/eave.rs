use std::{os::fd::{FromRawFd, AsRawFd}, fs::File, io::{Read, Write}};

fn main() {
  let mut sout = unsafe { File::from_raw_fd(std::io::stdout().as_raw_fd()) };
  let mut sin = unsafe { File::from_raw_fd(std::io::stdin().as_raw_fd()) };

  let mut buf = [0u8;1024];

  loop {
    match sin.read(&mut buf) {
      Ok(cnt) if cnt > 0 => {
        //let data = &buf[..cnt];
        write_byte_file(&buf[..cnt]);
        //let data = &buf[..cnt];
        sout.write(&buf[..cnt]).unwrap();
      }
      Ok(_) => {},
      Err(e) => {
        write_string_file(&format!("error: {}", e));
        break;
      }
    }
  }

}

fn write_byte_file(data: &[u8]) {
  write_string_file(&format!("bytes: {:?}", data));

}

fn write_string_file(s: &str) {
  std::fs::write("pipe.txt", s).expect("Unable to write file");
}
