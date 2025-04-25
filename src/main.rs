use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
use std::io::{Error, ErrorKind};
use std::time::{Instant, Duration};

fn main() {
    let args = std::env::args().skip(1);

    let mut out = false;
    let mut reps: u32 = 1;
    for arg in args{
        if arg.starts_with('-'){
            match arg.as_str(){
                "-help" => println!("Help coming soon!"),
                "-out" => out = true,
                _ => {
                    if let Some(val) = arg.strip_prefix("-n="){
                        reps = val.parse::<u32>().unwrap();
                    }
                }, 

            }
        }
        else{
            let mut total_duration: Duration = Duration::ZERO;
            for _ in 0..reps{
                let output_file: Stdio = if out {OpenOptions::new().create(true).append(true).open("simr_out.txt").unwrap().into()} else {Stdio::null()};
                match run_executable(&arg, &[].to_vec(), output_file){
                    Ok(time) => {
                        total_duration += time;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }

            print_results(reps, total_duration);

        }
    }
}


fn run_executable(path: &String, args: &Vec<String>, output_file: Stdio) -> Result<Duration, Error>{
    if path.ends_with(".py"){
        let mut new_args:Vec<String> = vec![(*path).clone()];
        new_args.extend((*args).clone());
        return run_executable(&"python3".to_string(), &new_args, output_file);
    }
    let start_time = Instant::now();
    let status = Command::new(path).args(args).stdout(output_file).status()?;
    let finish_time = start_time.elapsed();
    // let as_ms = finish_time.as_millis();
    // println!("took {as_ms}ms time");
    if status.success(){
        Ok(finish_time)
    }
    else{
        Err(Error::new(ErrorKind::Other, format!("Call ended with status {}",status)))
    }
}

fn print_results(reps: u32, total_duration: Duration) {
    let total_s = total_duration.as_secs_f64();
    let total_ms = total_duration.as_millis();
    let total_mus = total_duration.as_micros();
    let avg_s = total_s / reps as f64;
    let avg_ms = total_ms / reps as u128;
    let avg_mus = total_mus / reps as u128;
    println!("Runs executed: {reps}");
    println!("Total runtime: {total_s} seconds");
    println!("Average runtime:");
    if avg_s > 0.2 {println!("{avg_s} seconds");}
    if avg_ms > 10 && avg_ms < 10000 {println!("{avg_ms} milliseconds");}
    if avg_mus < 10000 {println!("{avg_mus} microseconds");}
}
