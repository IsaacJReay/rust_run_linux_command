mod funct;

fn main() {
    
    // Give Path Arguments for ls command
    let path: &str = "/test";

    //run foreground command with reading thorough output
    let (code, output, error) = funct::sudomkdir(path);

    match &code {
        1 => println!("{}", &error),
        0 => println!("{}", &output),
        _ => println!("Broken"),
    }

    let (code, output, error) = funct::ls(path);

    match &code {
        1 => println!("{}", &error),
        0 => println!("{}", &output),
        _ => println!("Broken"),
    }


    // run background command with PID returning
    println!("{}",funct::backgroundls(path));

}