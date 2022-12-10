use std::fs;

struct Machine {
    start: u32,
    period: u32,
    cycles: u32,
    sum_strengths: u32,
    regx: i32,
}

fn sprite_is_visible(cpu: &mut Machine) -> bool {
    let mut is_visible = false;
    let x = cpu.cycles.wrapping_sub(1) % cpu.period;

    if x >= (cpu.regx as u32).wrapping_sub(1) && x <= cpu.regx as u32 + 1 {
        is_visible = true;
    }

    //println!("pixel: {x}, sprite: {}, visible: {is_visible}", cpu.regx);

    is_visible
}

fn draw_crt(cpu: &mut Machine) {
    if sprite_is_visible(cpu) {
        print!("#");
    } else {
        print!(".");
    }
    if cpu.cycles % cpu.period == 0 { print!("\n"); }
}

fn exec_cycle(cpu: &mut Machine) {
    cpu.cycles += 1;

    let is_period = (cpu.cycles > 20) && ((cpu.cycles - cpu.start) % cpu.period == 0);

    // part 2
    draw_crt(cpu);

    // part 1
    if cpu.cycles == cpu.start || is_period {
        cpu.sum_strengths += cpu.regx as u32 * cpu.cycles;
    }
}

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/10_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input.split("\n");
    let mut cpu = Machine {
        start: 20,
        period: 40,
        cycles: 0,
        sum_strengths: 0,
        regx: 1,
    };

    println!("[10] solution b:"); // "RGLRBZAU"

    for l in lines {
        let inst: Vec<&str> = l.split(" ").collect();
        let op = inst[0];
        
        if op == "noop" {
            exec_cycle(&mut cpu);
        } else if op == "addx" {
            let signal: i32 = inst[1].parse().unwrap();
            exec_cycle(&mut cpu);
            exec_cycle(&mut cpu);
            cpu.regx += signal;
        }
    }

    println!("\n[10] solution a: {}", cpu.sum_strengths);
}
