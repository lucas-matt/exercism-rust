use std::fs::File;
use std::io::Read;

static mut NAMES: Vec<String> = Vec::new();

pub struct Robot{
    name: String
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: gen_one()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = gen_one();
    }
}

fn gen_one() -> String {
    let mut name = gen();
    unsafe {
        while NAMES.contains(&name) {
            name = gen(); 
        }
        NAMES.push(name.clone());
    }
    name
}

fn gen() -> String {
    let letters:Vec<String> = ('A'..='Z')
        .map(|c| c.to_string())
        .collect();
    let first = letters.get(rand() % letters.len());
    let second = letters.get(rand() % letters.len());
    format!("{}{}{:0>3}", first.unwrap(), second.unwrap(), rand().to_string()).to_string()
}

fn rand() -> usize {
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf).unwrap();
    (((buf[0] as usize) << 8) + buf[1] as usize) % 999
}

