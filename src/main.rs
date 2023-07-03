use std::collections::VecDeque;
use std::io::{BufWriter, Write};
const PATH: &str = "log.txt";
use std::time::Duration;
mod logc;
use std::fs::OpenOptions;

fn write_to_file(path: &str, data: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data.as_bytes())?;
    writer.write_all(b"\n")?;
    println!("write to file");

    Ok(())
}

fn main() {
    let mut time = 0;
    let mut iteration = 0;
    let mut log_vec: VecDeque<String> = VecDeque::with_capacity(10);
    loop {
        std::thread::sleep(Duration::from_secs(1));
        time += 5;
        iteration += 1;

        println!("Time: {}", time);
        println!("Iteration: {}", iteration);
        if time == 60 {
            let remaing_values = log_vec.iter().collect::<Vec<_>>();
            for i in remaing_values.iter() {
                write_to_file(PATH, i.as_str()).unwrap();
            }
            break;
        }

        let create_log = logc::new_log();
        log_vec.push_back(create_log);
        println!("{:?}", log_vec.len());

        if log_vec.len() == 10 {
            let retrieved_log = log_vec.pop_front();
            if let Some(retrieved_log) = retrieved_log {
                write_to_file(PATH, retrieved_log.as_str()).unwrap();
            } else {
                write_to_file(PATH, logc::corrupted_log().as_str()).unwrap();
            }
        }
    }
}



