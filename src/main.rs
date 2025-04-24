fn handle_flag(flag: String) -> Option<String>{
    match flag.as_str(){
        "-help" => Some("Help coming soon!".to_string()),
        _ => None, 
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
    }
}
