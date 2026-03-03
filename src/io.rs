use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write}; 
use std::path::PathBuf;

pub enum InputSource {
    File(File),
    Stdin(io::Stdin),
}

impl InputSource {
    pub fn new(path: Option<PathBuf>) -> io::Result<Self> {
        match path {
            Some(p) => Ok(InputSource::File(File::open(p)?)),
            None => Ok(InputSource::Stdin(io::stdin())),
        }
    }

    pub fn into_buffered(self) -> BufReader<Self> {
        BufReader::new(self) 
    }
}

impl Read for InputSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            InputSource::File(f) => f.read(buf),
            InputSource::Stdin(s) => s.read(buf),
        }
    }
}

pub enum OutputSource {
    File(File),
    Stdout(io::Stdout),
}

impl OutputSource {
    pub fn new(path: Option<PathBuf>) -> io::Result<Self> {
        match path {
            Some(p) => Ok(OutputSource::File(File::create(p)?)),
            None => Ok(OutputSource::Stdout(io::stdout())), 
        }
    }

    pub fn into_buffered(self) -> BufWriter<Self> {
        BufWriter::new(self)
    }
}

impl Write for OutputSource {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            OutputSource::File(f) => f.write(buf),
            OutputSource::Stdout(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputSource::File(f) => f.flush(),
            OutputSource::Stdout(s) => s.flush(),
        }
    }
}