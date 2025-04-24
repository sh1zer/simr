use std::io::{Error, ErrorKind};
use std::time::{Instant, Duration};


fn run_executable(path: &String, args: &Vec<String>) -> Result<Duration, Error>{
    if path.ends_with(".py"){
        let mut new_vec:Vec<String> = vec![(*path).clone()];
        new_vec.extend((*args).clone());
        return run_executable(&"python3".to_string(), &new_vec);
    }

    let start_time = Instant::now();
    let status = std::process::Command::new(path).args(args).status()?;
    let finish_time = start_time.elapsed();
    let as_ms = finish_time.as_millis();
    println!("took {as_ms}ms time");
    if status.success(){
        Ok(finish_time)
    }
    else{
        Err(Error::new(ErrorKind::Other, "Call ended with status {status}"))
    }
}

fn main() {
    #[cfg(windows)]
    println!("Windows not supported :(");
    
    #[cfg(unix)]
    println!("Hello, world!");

    let args = std::env::args().skip(1);

    let mut reps: u32 = 1;
    for arg in args{
        println!("#handling {arg}");

        if arg.starts_with('-'){
            match arg.as_str(){
                "-help" => println!("Help coming soon!"),
                _ => {
                    if let Some(val) = arg.strip_prefix("-n="){
                        reps = val.parse::<u32>().unwrap();
                    }
                }, 

            }
        }
        else{
            for _ in 0..reps{
                let _ = run_executable(&arg, &[].to_vec());
            }
        }
    }
}
