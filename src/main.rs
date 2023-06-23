use std::{io::Write, thread, time::Duration};

use clap::Parser;

type BPM = f32;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "BPM", default_value_t = 60.0)]
    tempo: BPM,
    #[arg(short, long = "bpb", default_value_t = 4)]
    beats_per_bar: usize,
}

fn main() {
    let args = Args::parse();

    let sec_per_beat = 60.0 / args.tempo;

    loop {
        for _ in 0..args.beats_per_bar {
            print!(".");
            std::io::stdout().flush().expect("Error flushing stdout");
            thread::sleep(Duration::from_secs_f32(sec_per_beat));
        }
        println!();
    }
}
