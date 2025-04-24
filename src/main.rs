use std::io::{Error, ErrorKind};

fn handle_flag(flag: String) -> Option<String>{
    match flag.as_str(){
        "-help" => Some("Help coming soon!".to_string()),
        _ => None, 
    }
}

fn run_executable(path: String, args: Vec<String>) -> Result<String, std::io::Error>{
    if path.ends_with(".py"){
        let status = std::process::Command::new("python3").args([path].into_iter().chain(args.into_iter())).status()?;
        if status.success(){
            return Ok("Hip hip hurray".to_string());
        }
        else{
            return Err(Error::new(ErrorKind::Other, "Call ended with status {status}"));
        }
    }
    let status = std::process::Command::new(path).args(args).status()?;
    if status.success(){
        Ok("Hip hip hurray".to_string())
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
