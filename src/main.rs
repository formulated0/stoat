use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, exit};
mod lexer;
use crate::lexer::*;

enum Op {
    Push(i32),
    Plus,
    Minus,
    Dump,
}

impl Op {
    pub const COUNT: usize = 4;
}

fn simulate_program(program: Vec<Op>) {
    assert!(
        Op::COUNT == 4,
        "exhaustive handling of operations in simulation"
    );
    let mut stack: Vec<i32> = vec![];
    for op in program {
        match op {
            Op::Push(value) => stack.push(value),
            Op::Plus => {
                let a = stack.pop().expect("stack underflow");
                let b = stack.pop().expect("stack underflow");
                stack.push(a + b);
            }
            Op::Minus => {
                let a = stack.pop().expect("stack underflow");
                let b = stack.pop().expect("stack underflow");
                stack.push(b - a);
            }
            Op::Dump => {
                let a = stack.pop().expect("stack underflow");
                println!("{}", a);
            }
        }
    }
}

fn compile_program(program: Vec<Op>, out_path: String) {
    let path = Path::new(&out_path);
    let display = path.display();

    let mut out = match File::create(&path) {
        Err(e) => panic!("couldnt create file {}: {}", display, e),
        Ok(out) => out,
    };

    out.write("section .text\n".as_bytes()).unwrap();

    out.write("dump:\n".as_bytes()).unwrap();
    out.write("    mov     r9, -3689348814741910323\n".as_bytes())
        .unwrap();
    out.write("    sub     rsp, 40\n".as_bytes()).unwrap();
    out.write("    mov     BYTE [rsp+31], 10\n".as_bytes())
        .unwrap();
    out.write("    lea     rcx, [rsp+30]\n".as_bytes()).unwrap();
    out.write(".L2:\n".as_bytes()).unwrap();
    out.write("    mov     rax, rdi\n".as_bytes()).unwrap();
    out.write("    lea     r8, [rsp+32]\n".as_bytes()).unwrap();
    out.write("    mul     r9\n".as_bytes()).unwrap();
    out.write("    mov     rax, rdi\n".as_bytes()).unwrap();
    out.write("    sub     r8, rcx\n".as_bytes()).unwrap();
    out.write("    shr     rdx, 3\n".as_bytes()).unwrap();
    out.write("    lea     rsi, [rdx+rdx*4]\n".as_bytes())
        .unwrap();
    out.write("    add     rsi, rsi\n".as_bytes()).unwrap();
    out.write("    sub     rax, rsi\n".as_bytes()).unwrap();
    out.write("    add     eax, 48\n".as_bytes()).unwrap();
    out.write("    mov     BYTE [rcx], al\n".as_bytes())
        .unwrap();
    out.write("    mov     rax, rdi\n".as_bytes()).unwrap();
    out.write("    mov     rdi, rdx\n".as_bytes()).unwrap();
    out.write("    mov     rdx, rcx\n".as_bytes()).unwrap();
    out.write("    sub     rcx, 1\n".as_bytes()).unwrap();
    out.write("    cmp     rax, 9\n".as_bytes()).unwrap();
    out.write("    ja      .L2\n".as_bytes()).unwrap();
    out.write("    lea     rax, [rsp+32]\n".as_bytes()).unwrap();
    out.write("    mov     edi, 1\n".as_bytes()).unwrap();
    out.write("    sub     rdx, rax\n".as_bytes()).unwrap();
    out.write("    lea     rsi, [rsp+32+rdx]\n".as_bytes())
        .unwrap();
    out.write("    mov     rdx, r8\n".as_bytes()).unwrap();
    out.write("    mov     rax, 1\n".as_bytes()).unwrap();
    out.write("    syscall\n".as_bytes()).unwrap();
    out.write("    add     rsp, 40\n".as_bytes()).unwrap();
    out.write("    ret\n".as_bytes()).unwrap();

    out.write("global _start\n".as_bytes()).unwrap();
    out.write("_start:\n".as_bytes()).unwrap();
    for op in program {
        assert!(Op::COUNT == 4, "exhaustive handling of ops in compilation");

        match op {
            Op::Push(a) => {
                out.write(format!("    ;; -- push {} --\n", a).as_bytes())
                    .unwrap();
                out.write(format!("    push {}\n", a).as_bytes()).unwrap();
            }
            Op::Plus => {
                out.write(format!("    ;; -- plus --\n").as_bytes())
                    .unwrap();
                out.write(format!("    pop rax\n").as_bytes()).unwrap();
                out.write(format!("    pop rbx\n").as_bytes()).unwrap();
                out.write(format!("    add rax, rbx\n").as_bytes()).unwrap();
                out.write(format!("    push rax\n").as_bytes()).unwrap();
            }
            Op::Minus => {
                out.write(format!("    ;; -- minus --\n").as_bytes())
                    .unwrap();
                out.write(format!("    pop rax\n").as_bytes()).unwrap();
                out.write(format!("    pop rbx\n").as_bytes()).unwrap();
                out.write(format!("    sub rbx, rax\n").as_bytes()).unwrap();
                out.write(format!("    push rbx\n").as_bytes()).unwrap();
            }
            Op::Dump => {
                out.write(format!("    ;; -- dump --\n").as_bytes())
                    .unwrap();
                out.write(format!("    pop rdi\n").as_bytes()).unwrap();
                out.write(format!("    call dump\n").as_bytes()).unwrap();
            }
        }
    }
    out.write("    mov rax, 60\n".as_bytes()).unwrap();
    out.write("    mov rdi, 0\n".as_bytes()).unwrap();
    out.write("    syscall\n".as_bytes()).unwrap();

    run_cmd("nasm", &["-felf64", &out_path]);
    run_cmd("ld", &["-o", "output", "output.o"]);
}

fn run_cmd(program: &str, args: &[&str]) {
    println!("running: {} {}", program, args.join(" "));
    Command::new(program).args(args).status().unwrap();
}

fn parse_tok_as_op(file_path: &str, line_idx: usize, col: usize, tok: &str) -> Op {
    assert!(
        Op::COUNT == 4,
        "exhaustive handling of ops in parse_tok_as_op"
    );
    match tok {
        "+" => Op::Plus,
        "-" => Op::Minus,
        "." => Op::Dump,
        _ => {
            let word: i32 = tok.parse().unwrap_or_else(|_| {
                eprintln!(
                    "{}:{}:{}: unknown token '{}'",
                    file_path, line_idx, col, tok
                );
                exit(1);
            });
            Op::Push(word)
        }
    }
}

fn load_program_from_file(file_path: String) -> Vec<Op> {
    let mut vec_ops: Vec<Op> = vec![];
    for (file_path, line_idx, col, tok) in lex_file(&file_path) {
        vec_ops.push(parse_tok_as_op(&file_path, line_idx, col, &tok));
    }
    vec_ops
}

fn usage() {
    eprintln!("usage: stoat <subcommand> [args]");
    eprintln!("subcommands: ");
    eprintln!("    sim <file>       simulate the program");
    eprintln!("    com <file>       compile the program");
}

fn main() {
    let mut argv = args();

    if argv.len() < 2 {
        eprintln!("err: no subcommand provided");
        usage();
        exit(1);
    }

    let subcommand = argv.nth(1).unwrap();

    if subcommand == "sim" {
        match argv.nth(0) {
            Some(program_path) => {
                println!("sim file: {}", program_path); // TODO: remove debug text
                let program = load_program_from_file(program_path);
                simulate_program(program);
            }
            None => {
                eprintln!("err: no input file is provided for the simulation");
                usage();
                exit(1)
            }
        }
    } else if subcommand == "com" {
        match argv.nth(0) {
            Some(program_path) => {
                println!("com file: {}", program_path); // TODO: remove debug text
                let program = load_program_from_file(program_path);
                compile_program(program, String::from("output.asm"));
            }
            None => {
                eprintln!("err: no input file is provided for the simulation");
                usage();
                exit(1)
            }
        }
    } else {
        eprintln!("err: unknown subcommand {}", subcommand);
        usage();
        exit(1);
    }
}
