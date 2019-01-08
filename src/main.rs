extern crate rustyline;

mod mix_util;
mod mix_types;
mod mix_core;

use self::mix_types::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn handle_input(line: String, st: &mut MixState) -> () {
    let words = line.trim().split(" ").collect::<Vec<_>>();
    // For now, ignore all bad commands.
    match words.len() {
        0 => {},
        1 => {
            if words[0] == "run" || words[0] == "r" {
                mix_core::do_mix_run(st);
            } else if words[0] == "step" || words[0] == "s" {
                mix_core::do_mix_step(st);
            } else  {
                println!("Bad command");
        },
        2 => {
            if words[0] == "show" {
                if words[1] == "registers" || words[1] == "r" {
                    println!("{:#?}", st.r);
                } else if words[1] == "state" || words[1] == "st" {
                    println!("{:#?}", st);
                } else if words[1] == "instructions" || words[1] == "i" {
                    // TODO: nice disassembled instructions
                    println!("{:#?}", st.pc);
                    let lb = std::max(0, st.pc as usize - 10);
                    let ub = std::min(MEM_SIZE - 1, st.pc as usize + 10);
                    println!("{:#?}", &st.memory[lb .. ub]);
                }
                else {
                    println!("Bad command");
                }
            } else {
                println!("Bad command");
            }
        }
        3 => {} // TODO: set registers, set memory, show memory,
                // load from file into memory starting at position.
        4 => {} // TODO: show memory range
        _ => println!("Bad command");
    }
    // TODO: fancier commands: breakpoints and watchpoints.
    // (can check in memory get, memory set, and step/jump)
}

fn main() {
    // Initialize MIX state
    let mut mem: MixMemory = [0; MEM_SIZE];
    let mut st = MixState {
        r: MixRegisters {
            a: 0,
            x: 0,
            i: [0, 0, 0, 0, 0, 0],
            j: 0,
        },
        comparison: MixCompare::Equal,
        overflow: false,
        memory: &mem,
        io: (),
        pc: 0,
        is_running: false,
        panic_msg: None,
    };

    // Readline loop

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                handle_input(line, &mut st);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
