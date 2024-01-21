use std::{os::fd::{FromRawFd, AsRawFd}, fs::{File, OpenOptions}, io::{Read, Write}, process::{Command, Stdio}};

fn main() {
  let mut eave = Command::new("target/debug/reqbug")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  let mut req_sout = eave.stdout.take().unwrap();
  let mut req_sin = eave.stdin.take().unwrap();

  let mut sout = unsafe { File::from_raw_fd(std::io::stdout().as_raw_fd()) };
  let mut sin = unsafe { File::from_raw_fd(std::io::stdin().as_raw_fd()) };

  let mut buf_in = [0u8;1024];
  let mut buf_out = [0u8;1024];

  loop {
    match req_sout.read(&mut buf_out) {
      Ok(cnt) if cnt > 0 => {
        write_byte_file(&buf_out[..cnt]);
        sout.write(&buf_out[..cnt]).unwrap();
      }
      Ok(_) => {},
      Err(e) => {
        write_string_file(&format!("error_out: {}", e));
        break;
      }
    }
    match sin.read(&mut buf_in) {
      Ok(cnt) if cnt > 0 => {
        req_sin.write(&buf_in[..cnt]).unwrap();
      }
      Ok(_) => {},
      Err(e) => {
        write_string_file(&format!("error_in: {}", e));
        break;
      }
    }
  }

}

fn write_byte_file(data: &[u8]) {
  write_string_file(&format!("bytes: {:?}", data));

}

fn write_string_file(s: &str) {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("pipe.txt")
    .unwrap();
  writeln!(file, "{}", s).expect("Unable to write file");
}
