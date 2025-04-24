use std::io::{Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};

fn handle_flag(flag: String) -> Option<String>{
    match flag.as_str(){
        "-help" => Some("Help coming soon!".to_string()),
        _ => None, 
    }
}

fn run_executable(path: String, args: Vec<String>) -> Result<u128, std::io::Error>{
    if path.ends_with(".py"){
        let mut new_vec:Vec<String> = vec![path];
        new_vec.extend(args);
        return run_executable("python3".to_string(), new_vec);
    }

    let start_time = Instant::now();
    let status = std::process::Command::new(path).args(args).status()?;
    let finish_time = start_time.elapsed().as_millis();
    println!("took {finish_time}ms time");
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
    for arg in args{
        println!("#handling {arg}");

        if arg.starts_with('-'){
            match handle_flag(arg){
                Some(ret) => println!("{ret}"),
                None => println!("Invalid flag"),
            }
        }

        else{
            let _ = run_executable(arg, [].to_vec());
        }
    }
}
