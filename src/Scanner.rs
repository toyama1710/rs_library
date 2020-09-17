#[allow(dead_code)]

use::std::io::stdin;
use::std::io::Read;
use::std::str::FromStr;
use::std::fmt::Debug;

pub struct Scanner {
    idx: usize, 
    buf: Vec<String>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Self {
            idx:0,
            buf: {
                let mut s = String::new();
                stdin().read_to_string(&mut s).ok();
                s.split_whitespace().map(|x| x.to_owned()).collect()
            },
        }
    }

    fn read<T: FromStr> (&mut self) -> T 
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        if self.idx >= self.buf.len() {
            panic!("reached the end of input")
        }
        let ret = self.buf[self.idx].parse::<T>().expect("parse error");
        self.idx += 1;
        return ret;
    }
}