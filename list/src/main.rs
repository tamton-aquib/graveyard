use std::path::Path;
use std::{env, fs, io};
mod icon;

fn real_main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() < 2 { "." } else { &args[1] };

    let entries = fs::read_dir(&dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut nice_list: Vec<String> = vec![];
    for entry in &entries {
        let filename = Path::new(entry);
        let str_filename = &filename.file_name().unwrap().to_str().unwrap().to_string();

        let icon = icon::get_fileicon(filename);
        let item = format!("{} {}", icon, str_filename);

        nice_list.push(item);
    }
    println!(" {} ", nice_list.join("   "));

    Ok(())
}

fn main() {
    real_main().unwrap();
}
