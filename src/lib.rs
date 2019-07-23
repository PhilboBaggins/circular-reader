#[allow(dead_code)]
pub struct CircularReader {
    pos: usize,
    buf: Vec<usize>,
}

impl CircularReader {
    #[allow(dead_code)]
    pub fn new(buf: Vec<usize>) -> CircularReader {
        CircularReader { pos: 0, buf: buf }
    }
}

impl Iterator for CircularReader {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.buf[self.pos];
        self.pos += 1;
        if self.pos >= self.buf.len() {
            self.pos = 0;
        }
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec!(1, 2, 3);
        let mut cr = CircularReader::new(vec);
        assert_eq!(cr.next(), Some(1));
        assert_eq!(cr.next(), Some(2));
        assert_eq!(cr.next(), Some(3));
        assert_eq!(cr.next(), Some(1));
        assert_eq!(cr.next(), Some(2));
        assert_eq!(cr.next(), Some(3));
    }
}
