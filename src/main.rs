use std::path::Path;
use std::fs;
use std::io::{BufRead,BufReader,BufWriter, Write};

const LINE_BREAK: usize = 10000;

fn main() {
    println!("Hello, world!");

    let file_path = Path::new("リスト.txt");
    let in_file = fs::File::open(file_path).unwrap();
    let bufred = BufReader::new(in_file);
    let lines = bufred.lines();
    let mut lines = lines.filter_map(|x|x.ok());

    let mut counter = 0;

    let count= lines.by_ref().count();
    let file_count = count / LINE_BREAK;

    let mut out_files = Vec::new();
    for out_file in 0..file_count {
        let file_name = format!("処置_List_{}.txt", out_file.to_string());
        out_files.push(BufWriter::new(fs::File::create(file_name).unwrap()));
    }

    for line in lines {

        write!(out_files[counter / LINE_BREAK], "{}\r\n", line ).unwrap();
        counter = counter + 1;

    }

    for mut b in out_files {
        b.flush().unwrap();
    }

}
