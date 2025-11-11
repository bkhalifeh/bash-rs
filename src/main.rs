use std::{
    collections::HashMap,
    io::{BufRead, Write},
    path::PathBuf,
    process,
};

fn get_current_directory() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::new())
        .to_str()
        .unwrap_or_else(|| "")
        .to_string()
}

fn get_current_username() -> String {
    return whoami::username();
}

fn get_hostname() -> String {
    return whoami::devicename();
}

fn print_command_line() {
    let username = get_current_username();
    let hostname = get_hostname();
    let directory = get_current_directory();
    print!("{username}@{hostname}:{directory}$ ");
    std::io::stdout().flush().unwrap_or_else(|_| {
        return ();
    })
}

fn get_input() -> Vec<String> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap_or_default();
    buf.trim()
        .to_string()
        .split(" ")
        .map(|s| s.to_string())
        .collect()
}

fn command_pwd(_: &[String]) -> Result<(), std::io::Error> {
    println!("{}", get_current_directory());
    Ok(())
}

fn command_whoami(_: &[String]) -> Result<(), std::io::Error> {
    println!("{}", get_current_username());
    Ok(())
}

fn command_cd(args: &[String]) -> Result<(), std::io::Error> {
    if args.len() > 0 && args.len() == 2 {
        let new_directory = &args[1];
        std::env::set_current_dir(new_directory).unwrap_or_else(|_| {});
    }
    Ok(())
}

fn command_hostname(_: &[String]) -> Result<(), std::io::Error> {
    println!("{}", get_hostname());
    Ok(())
}

fn command_exit(_: &[String]) -> Result<(), std::io::Error> {
    process::exit(0);
}

fn command_ls(args: &[String]) -> Result<(), std::io::Error> {
    let path = if args.len() <= 1 {
        ".".to_owned()
    } else {
        args[1].to_string()
    };
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            println!("📁 {}", path.display());
        } else if metadata.is_file() {
            println!("📄 {}", path.display());
        } else {
            println!("❓ {}", path.display());
        }
    }
    Ok(())
}

fn command_cat(args: &[String]) -> Result<(), std::io::Error> {
    if args.len() == 2 {
        let file_path = args[1].to_string();
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn command_clear(_: &[String]) -> Result<(), std::io::Error> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()?;
    } else {
        std::process::Command::new("clear").status()?;
    }

    Ok(())
}

fn command_rm(args: &[String]) -> Result<(), std::io::Error> {
    if args.len() == 2 {
        let file_path = args[1].to_string();
        std::fs::remove_file(file_path)?;
    }
    Ok(())
}

fn command_rmdir(args: &[String]) -> Result<(), std::io::Error> {
    if args.len() == 2 {
        let directory_path = args[1].to_string();
        std::fs::remove_dir_all(directory_path)?;
    }
    Ok(())
}

fn command_mkdir(args: &[String]) -> Result<(), std::io::Error> {
    if args.len() == 2 {
        let directory_path = args[1].to_string();
        std::fs::create_dir_all(directory_path)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut command_handlers: HashMap<String, fn(&[String]) -> Result<(), std::io::Error>> =
        HashMap::new();

    command_handlers.insert("pwd".to_owned(), command_pwd);
    command_handlers.insert("whoami".to_owned(), command_whoami);
    command_handlers.insert("cd".to_owned(), command_cd);
    command_handlers.insert("hostname".to_owned(), command_hostname);
    command_handlers.insert("exit".to_owned(), command_exit);
    command_handlers.insert("ls".to_owned(), command_ls);
    command_handlers.insert("cat".to_owned(), command_cat);
    command_handlers.insert("clear".to_owned(), command_clear);
    command_handlers.insert("rm".to_owned(), command_rm);
    command_handlers.insert("rmdir".to_owned(), command_rmdir);
    command_handlers.insert("mkdir".to_owned(), command_mkdir);

    loop {
        print_command_line();
        let input = get_input();
        if input.len() > 0 {
            let command_name = input[0].to_string();
            if let Option::Some(handler) = command_handlers.get(&command_name) {
                let _ = handler(&input);
            }
        }
    }
}
