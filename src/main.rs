use std::{io::Write, env};
use std::fs::File;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};

use mp3_duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let audio = &args[2];

    print!("reading... ");
    let contents = File::open(filename).expect("fuck");
    println!("done.");

    print!("finding red lines... ");

    // find timing points
    let mut points: Vec<String> = Vec::new();
    let reader = BufReader::new(contents);
    let mut collect = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if collect {
            points.push(line.to_string());
        }

        if line == "[TimingPoints]" {
            collect = true;
        }

        if line == "[Colours]" {
            points.resize(points.len() - 3, String::new());
            break;
        }
    }

    // find and set up red lines
    let mut reds: Vec<(i64, f32)> = Vec::new(); // first element of the tuple is the red line offset, 
    for point in points {                // second is the time between bars
        let elements: Vec<&str> = point.split(',').collect();
        if elements[6] == "1" {
            let offset: i64 = elements[0].parse().unwrap();
            let timesig: f32 = elements[2].parse().unwrap();
            let diff: f32 = elements[1].parse::<f32>().unwrap() * timesig; // multiply beat time against time sig to get bar time
            reds.push((offset, diff));
        }
    }
    println!("done.");

    // find the length of the song, probably wrong lol
    let songlength = mp3_duration::from_path(Path::new(audio)).expect("the fuck happened here").as_millis(); // pain
    reds.push((songlength as i64, 69420.0)); // hack to make my life easier


    print!("generating mods... ");
    let mut mods: String = String::new();

    // lmao
    fn mods_per_redline(start: i64, end: i64, bartime: f32, mods: &mut String) {
        for i in start..end {
            if i % (bartime.round() as i64) == 0 {
                let mins = i / 1000 / 60;
                let secs = i / 1000 - (mins * 60);
                let mils = i - (mins * 60 * 1000) - (secs * 1000);
                let entry = format!("{}:{}:{} NC.\n", mins, secs, mils);
                mods.push_str(entry.as_str());
            }
        }
    }

    let mut next = 0;
    for red in reds.clone() {
        if next == reds.len() - 1 {
            break;
        }
        mods_per_redline(red.0, reds[next + 1].0, red.1, &mut mods);
        next += 1;
    }
    println!("done.");

    // write to file
    print!("writing mods... ");

    let mut file = File::create("mods.txt").expect("fuck me");
    file.write_all(mods.as_bytes()).expect("fuck you");
    println!("done.");
    println!("enjoy your kudosu lmao");
}
