use std::io::{Read, Result, Write};
use std::marker::PhantomData;

pub struct ReadStats<R> {
    resource: R,
    resource_type: PhantomData<R>,
    reads: usize,
    bytes: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            resource: wrapped,
            resource_type: PhantomData,
            reads: 0,
            bytes: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.resource
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        match self.resource.read(buf) {
            Ok(size) => {
                self.bytes += size;
                Ok(size)
            },
            err => err
        }
    }

}

pub struct WriteStats<W> {
    resource: W,
    resource_type: PhantomData<W>,
    writes: usize,
    bytes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            resource: wrapped,
            resource_type: PhantomData,
            writes: 0,
            bytes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.resource
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        match self.resource.write(buf) {
            Ok(size) => {
                self.bytes += size;
                Ok(size)
            },
            err => err
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.resource.flush()
    }
}
