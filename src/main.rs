mod cpu;

use cpu::Cpu;
use std::env;
use std::fs;

fn parse_hex(src: &str) -> Vec<u8> {
    let mut prog = Vec::new();
    for line in src.lines() {
        let line = match line.find(';') {
            Some(i) => &line[..i],
            None => line,
        };
        for c in line.chars() {
            if let Some(d) = c.to_digit(16) {
                prog.push(d as u8);
            }
        }
    }
    prog
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let prog = if args.len() > 1 {
        let src = fs::read_to_string(&args[1]).expect("couldnt read file");
        parse_hex(&src)
    } else {
        parse_hex("1 0  D  E 1  C 0 B  A 0 2  0 0 F")
    };

    let debug = args.iter().any(|a| a == "-d" || a == "--debug");

    let mut cpu = Cpu::new();
    cpu.load(&prog);

    let max = 10000;
    while !cpu.halted && cpu.cycles < max {
        if debug {
            println!("pc={:02X} op={:X} a={:X}", cpu.pc, cpu.mem[cpu.pc as usize], cpu.a);
        }
        cpu.step();
    }

    if !cpu.halted {
        println!("didnt halt after {} cycles, giving up", max);
    }

    println!();
    cpu.dump();
}
