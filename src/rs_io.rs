#[allow(dead_code)]

pub struct Scanner {
    idx: usize, 
    buf: Vec<String>,
}

impl Scanner {
    pub fn new<T: std::io::Read>(inf: &mut T) -> Scanner {
        Self {
            idx:0,
            buf: {
                let mut s = String::new();
                inf.read_to_string(&mut s).expect("I/O error");
                s.split_whitespace().map(|x| x.to_owned()).collect()
            },
        }
    }

    pub fn read<T: std::str::FromStr> (&mut self) -> T 
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        if self.empty() {
            panic!("reached the end of input")
        }
        let ret = self.buf[self.idx].parse::<T>().expect("parse error");
        self.idx += 1;
        return ret;
    }

    pub fn empty(&self) -> bool {
        return self.idx >= self.buf.len();
    }
}