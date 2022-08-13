use std::{fs, path::Path};

fn read_lines(filename: &Path) -> (usize, usize) {
    let content = fs::read_to_string(filename).expect("Couldnt Open file");
    let empty_count = content.lines().filter(|l| l.is_empty()).count();

    let total_lines = content.lines().count();
    let without_empty = total_lines - empty_count;

    (total_lines, without_empty)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify a filename!");
        return;
    }

    let filename = Path::new(&args[1]);
    if filename.is_file() {
        let (total, without) = read_lines(filename);
        println!("Total: {}\nNon-Empty: {}", total, without);
    } else {
        let dirs = fs::read_dir(filename).expect("Could not read directory!");
        let (mut total, mut without) = (0, 0);

        for file in dirs {
            let file = file.unwrap().path();
            let (tot, wo) = read_lines(&file);
            total += tot;
            without += wo;
        }
        println!("Total: {}\nNon-Empty: {}", total, without);
    }
}
