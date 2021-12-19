use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify a filename!");
        return;
    }

    let filename = &args[1];
    let mut count: u32 = 0;

    let content = fs::read_to_string(filename).expect("Couldnt Open file");
    let lines = content.split("\n");
    // let lines = content.split("\n"); //.map(|s| count=count+1);

    for line in lines.into_iter() {
        if line != "" {
            count = count + 1;
        }
    }

    println!("{}", count);
}
