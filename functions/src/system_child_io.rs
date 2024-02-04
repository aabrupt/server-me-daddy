use std::{fs::File, io::Write};

pub enum IOStream {
    StdErr,
    StdIn,
    StdOut,
}
pub struct SystemChildIO {
    inner_hash: u64,
    dir_path: String,
}

impl SystemChildIO {
    pub(crate) fn new(inner_hash: u64, dir_path: String) -> Self {
        Self {
            inner_hash,
            dir_path,
        }
    }
    pub fn read_file(&self, stream: IOStream) -> Result<File, std::io::Error> {
        let file_extension = match stream {
            IOStream::StdErr => ".stderr",
            IOStream::StdIn => ".stdin",
            IOStream::StdOut => ".stdout",
        };
        let path = format!("{}/{}{}", self.dir_path, self.inner_hash, file_extension);
        File::open(path)
    }
    pub fn write_to_file(&self, stream: IOStream, content: &str) -> Result<(), std::io::Error> {
        let file_extension = match stream {
            IOStream::StdErr => ".stderr",
            IOStream::StdIn => ".stdin",
            IOStream::StdOut => ".stdout",
        };
        let path = format!("{}/{}{}", self.dir_path, self.inner_hash, file_extension);
        File::options()
            .append(true)
            .write(true)
            .open(path)?
            .write_all(content.as_bytes())
    }
}
