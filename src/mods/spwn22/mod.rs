use std::process::{exit, Command};
use std::io::{self, Write, Seek, SeekFrom};
use std::fs::{self, OpenOptions};
use colored::Colorize;

fn clearcmd() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}

fn error(error: &str) {
    println!("{}", error.red().bold());
}

fn warn(warn: &str) {
    println!("{}", warn.yellow().bold());
}

fn info(info: &str) {
    println!("{}", info.blue().bold());
}

fn yippee(yippee: &str) {
    println!("{}", yippee.green().bold());
}

fn change_specific_byte(address: u64, new_byte: u8, path: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    file.seek(SeekFrom::Start(address))?;
    file.write_all(&[new_byte])?;
    Ok(())
}

// Mod Features

fn modfeature1() -> Result<(), Box<dyn std::error::Error>> {
    info("Finding spwn.exe...");
    if let Ok(_) = fs::metadata("C:/Program Files/spwn/spwn.exe") {
        yippee("Found spwn.exe!");
        info("Reading spwn.exe...");
        match fs::read("C:/Program Files/spwn/spwn.exe") {
            Ok(spwncontent) => {
                yippee("Successfully read spwn.exe!");
                if spwncontent.get(0) == Some(&77) && spwncontent.get(1) == Some(&90) {
                    yippee("Your spwn.exe file is a correct .exe file! It doesn't have any problem!");
                    let _ = change_specific_byte(0x007E2F30, 0x27, "C:/Program Files/spwn/spwn.exe");
                    let _ = change_specific_byte(0x007E2F31, 0x0F, "C:/Program Files/spwn/spwn.exe");
                    yippee("SPWN was successfully modded!");
                    Ok(())
                } else {
                    warn("Warning: Your spwn.exe file is NOT a valid .exe file. This spwn.exe file is CORRUPTED.");
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Corrupted spwn.exe file")))
                }
            }
            Err(err) => {
                error("Error reading spwn.exe");
                Err(Box::new(err))
            }
        }
    } else {
        error("Error: spwn.exe doesn't exist. Make sure you installed SPWN correctly.");
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "spwn.exe not found")))
    }
}

fn modfeature2() -> Result<(), Box<dyn std::error::Error>> {
    info("Finding std/general_triggers.spwn...");
    if let Ok(_) = fs::metadata("C:/Program Files/spwn/libraries/std/general_triggers.spwn") {
        yippee("Found general_triggers.spwn!");
        info("Reading general_triggers.spwn...");
        match fs::read("C:/Program Files/spwn/libraries/std/general_triggers.spwn") {
            Ok(_) => {
                yippee("Successfully read general_triggers.spwn!");
                match fs::read("C:/Workspaces/mddr/src/mods/spwn22/newgeneral_triggers.spwn") {
                    Ok(newcontent) => {
                        match fs::write("C:/Program Files/spwn/libraries/std/general_triggers.spwn", newcontent) {
                            Ok(_) => {
                                yippee("SPWN was successfully modded!");
                                Ok(())
                            },
                            Err(err) => {
                                error("Error while adding end trigger");
                                Err(Box::new(err))
                            }
                        }
                    },
                    Err(err) => {
                        error("Error while reading newgeneral_triggers.spwn");
                        Err(Box::new(err))
                    }
                }
            },
            Err(err) => {
                error("Error reading general_triggers.spwn");
                Err(Box::new(err))
            }
        }
    } else {
        error("Error: general_triggers.spwn doesn't exist. Make sure you installed SPWN correctly.");
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "general_triggers.spwn not found")))
    }
}

// End of Mod Features

pub fn init() -> io::Result<()> {
    println!("SPWN 2.2");
    println!("1. Upgrade group limit (old spwn versions (<0.8) only)");
    println!("2. Add end trigger");
    println!("3. All above this");
    println!("4. Exit");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_string();
    match input.as_str() {
        "1" => {
            clearcmd();
            let _ = modfeature1();
            init()?;
        },
        "2" => {
            clearcmd();
            let _ = modfeature2();
            init()?;
        },
        "3" => {
            clearcmd();
            let _ = modfeature1();
            let _ = modfeature2();
            init()?;
        },
        "4" => {
            exit(0);
        },
        _ => {
            let _ = init()?;
        }
    }
    Ok(())
}