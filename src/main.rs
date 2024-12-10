use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of the challenge (1-25)
    day: u8,
    /// Part of the challenge (1 or 2)
    part: u8,
}

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

fn main() {
    let args = Args::parse();

    if args.day < 1 || args.day > 25 {
        eprintln!("Error: Day must be between 1 and 25.");
        std::process::exit(1);
    }

    if args.part != 1 && args.part != 2 {
        eprintln!("Error: Part must be either 1 or 2.");
        std::process::exit(1);
    }

    let input_file = format!("inputs/d{}_{}.txt", args.day, args.part);

    let result = match (args.day, args.part) {
        (1, 1) => d1::part1(&input_file),
        (1, 2) => d1::part2(&input_file),
        (2, 1) => d2::part1(&input_file),
        (2, 2) => d2::part2(&input_file),
        (3, 1) => d3::part1(&input_file),
        (3, 2) => d3::part2(&input_file),
        (4, 1) => d4::part1(&input_file),
        (4, 2) => d4::part2(&input_file),
        (5, 1) => d5::part1(&input_file),
        (5, 2) => d5::part2(&input_file),
        (6, 1) => d6::part1(&input_file),
        (6, 2) => d6::part2(&input_file),
        (7, 1) => d7::part1(&input_file),
        (7, 2) => d7::part2(&input_file),
        (8, 1) => d8::part1(&input_file),
        (8, 2) => d8::part2(&input_file),
        (9, 1) => d9::part1(&input_file),
        (9, 2) => d9::part2(&input_file),
        (10, 1) => d10::part1(&input_file),
        (10, 2) => d10::part2(&input_file),
        (11, 1) => d11::part1(&input_file),
        (11, 2) => d11::part2(&input_file),
        (12, 1) => d12::part1(&input_file),
        (12, 2) => d12::part2(&input_file),
        (13, 1) => d13::part1(&input_file),
        (13, 2) => d13::part2(&input_file),
        (14, 1) => d14::part1(&input_file),
        (14, 2) => d14::part2(&input_file),
        (15, 1) => d15::part1(&input_file),
        (15, 2) => d15::part2(&input_file),
        (16, 1) => d16::part1(&input_file),
        (16, 2) => d16::part2(&input_file),
        (17, 1) => d17::part1(&input_file),
        (17, 2) => d17::part2(&input_file),
        (18, 1) => d18::part1(&input_file),
        (18, 2) => d18::part2(&input_file),
        (19, 1) => d19::part1(&input_file),
        (19, 2) => d19::part2(&input_file),
        (20, 1) => d20::part1(&input_file),
        (20, 2) => d20::part2(&input_file),
        (21, 1) => d21::part1(&input_file),
        (21, 2) => d21::part2(&input_file),
        (22, 1) => d22::part1(&input_file),
        (22, 2) => d22::part2(&input_file),
        (23, 1) => d23::part1(&input_file),
        (23, 2) => d23::part2(&input_file),
        (24, 1) => d24::part1(&input_file),
        (24, 2) => d24::part2(&input_file),
        (25, 1) => d25::part1(&input_file),
        (25, 2) => d25::part2(&input_file),
        _ => Err(format!("Not implemented: d{} p{}", args.day, args.part)),
    };

    match result {
        Ok(res) => println!("Answer: {}", res),
        Err(e) => println!("No result provided. Error: {}", e),
    };
}
