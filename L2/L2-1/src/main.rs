use L2_1::Program;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let filename = args.last().unwrap().to_string();
    let mut program = Program::with_file_name(filename.clone());

    // if there no args, count words
    if args.len() == 2 {
        program.add_count_words();
    }

    // parse args, add functions
    for flag in args.iter().skip(1)  {
        if flag.eq("-c") {
            program.add_count_chars();
        } else if flag.eq("-l") {
            program.add_count_lines();
        } else if flag.eq("-w") {
            program.add_count_words();
        }
    }

    let output = program.run();

    for num in output.iter() {
        print!("{} ", num);
    }

    println!("{}", filename);
}
