use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify a filename!");
        return;
    }

    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("Couldnt Open file");
    let empty_count = content.lines().filter(|l| l.is_empty()).count();

    let total_lines = content.lines().count();
    let without_empty = total_lines - empty_count;

    println!("Total: {}\nNon-Empty: {}", total_lines, without_empty);
}
