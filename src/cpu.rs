pub const MEM_SIZE: usize = 256;

pub struct Cpu {
    pub a: u8,
    pub pc: u8,
    pub carry: bool,
    pub zero: bool,
    pub mem: [u8; MEM_SIZE],
    pub halted: bool,
    pub cycles: u64,
    pub out: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            pc: 0,
            carry: false,
            zero: false,
            mem: [0; MEM_SIZE],
            halted: false,
            cycles: 0,
            out: vec![],
        }
    }

    pub fn load(&mut self, prog: &[u8]) {
        for (i, n) in prog.iter().enumerate() {
            if i >= MEM_SIZE {
                break;
            }
            self.mem[i] = n & 0xF;
        }
    }

    fn fetch(&mut self) -> u8 {
        let v = self.mem[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        v
    }

    fn fetch_addr(&mut self) -> u8 {
        let hi = self.fetch();
        let lo = self.fetch();
        (hi << 4) | lo
    }

    fn set_a(&mut self, v: u8) {
        self.a = v & 0xF;
        self.zero = self.a == 0;
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }
        let op = self.fetch();
        self.cycles += 1;

        match op {
            0x0 => {}
            0x1 => {
                let n = self.fetch();
                self.set_a(n);
            }
            0x2 => {
                let addr = self.fetch_addr();
                let v = self.mem[addr as usize];
                self.set_a(v);
            }
            0x3 => {
                let addr = self.fetch_addr();
                self.mem[addr as usize] = self.a;
            }
            0x4 => {
                let addr = self.fetch_addr();
                let r = self.a + self.mem[addr as usize];
                self.carry = r > 0xF;
                self.set_a(r);
            }
            0x5 => {
                let addr = self.fetch_addr();
                let m = self.mem[addr as usize];
                self.carry = m > self.a;
                self.set_a(self.a.wrapping_sub(m));
            }
            0x6 => {
                let addr = self.fetch_addr();
                let v = self.a & self.mem[addr as usize];
                self.set_a(v);
            }
            0x7 => {
                let addr = self.fetch_addr();
                let v = self.a | self.mem[addr as usize];
                self.set_a(v);
            }
            0x8 => {
                let addr = self.fetch_addr();
                let v = self.a ^ self.mem[addr as usize];
                self.set_a(v)
            }
            0x9 => self.set_a(!self.a),
            0xA => {
                self.pc = self.fetch_addr();
            }
            0xB => {
                let addr = self.fetch_addr();
                if self.zero {
                    self.pc = addr;
                }
            }
            0xC => {
                let addr = self.fetch_addr();
                if self.carry {
                    self.pc = addr
                }
            }
            0xD => {
                println!("OUT: {:X} ({})", self.a, self.a);
                self.out.push(self.a);
            }
            0xE => {
                let n = self.fetch();
                let r = self.a + n;
                self.carry = r > 0xF;
                self.set_a(r);
            }
            0xF => self.halted = true,
            _ => unreachable!(),
        }
    }

    pub fn dump(&self) {
        println!(
            "A={:X} PC={:02X} C={} Z={} cycles={}",
            self.a, self.pc, self.carry as u8, self.zero as u8, self.cycles
        );
        for row in 0..16 {
            print!("{:02X}: ", row * 16);
            for col in 0..16 {
                print!("{:X} ", self.mem[row * 16 + col]);
            }
            println!();
        }
    }
}
