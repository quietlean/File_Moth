use std::io::{self};
use std::io::Write;
use std::{error::Error, fs::File};
use std::time::Duration;
//use csv_to_table::iter::CsvRecords;
use indicatif::ProgressBar;
use std::process::Command;
use std::path::{self, Path, PathBuf}; //, PathBuf
use std::fs;
use serde::Deserialize;
use toml_edit::{Array, DocumentMut, Item, Table, Value}; //, value
use mac_notification_sys::*; // Sending System Notifications
use dir;
//use rfd; // Opening Finder to select Folder
//use native_dialog::{DialogBuilder}; // Also a crate to display file choosers


//mod external;
use crate::external::com;

#[derive(Debug, Deserialize)]
pub struct Config {
    general: General,
    project: Project,
}

#[derive(Debug, Deserialize)]
struct General {
    amount: i64,
    daytime: Vec<i64>,
    weekday: Vec<i64>,
    monthday: Vec<i64>
}

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
struct Settings {
    notifications: i64,
    manual_input: i64
}

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct Project {
    pub name: Vec<String>,
    pub origin: Vec<String>,
    //destination: Vec<Vec<String>>,
    pub destination: Vec<Vec<Vec<String>>>,
    daemon: Vec<i64>
}

pub fn print_input(message: &str) -> String{

    //If the message is empty it wont be printed
    if message.len() > 0 {
        println!("{}", message);
    }

    print!(">  ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("Error while reading input: {}", error);
    }

    // trim creates a &str so you need to turn it into a String again
    input.trim().to_string()

    //if input.chars().any(char::is_whitespace) == true {
    //    println!("++ containing spaces -> not valid");
    //    return "error".to_string();
    //} else {
    //    return input;
    //}
}

pub fn menu_print_intro(){
    //User options are printed.
    //let mut count = String::new();
    let count = match open_toml() {
        Ok(config) => {
            //string to int
            config.general.amount
        },
        Err(()) => {
            println!("error while opening config.");
            0
        },
    };

    //let count_int: u32 = count.parse().unwrap();

    // Depending on the magnitude of "amount" I have to change the amount of spaces in the line where the "amount" is printed.
    // If amount gets bigger then 9 (x > 10 takes two spaces "--") I need less spaces ... the same with 100
    // Amount should be limited to 999 or the output will bug if amount is > 999
    let mut spaces = "";
    if count < 10 {
        spaces = "  ";
    } else if count < 100 {
        spaces = " ";
    }

    // Escape Codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    // ASCII Symboles: https://www.ee.ucl.ac.uk/mflanaga/java/HTMLandASCIItableC1.html
    println!("\n");
    println!("\x1b[48;5;225;30m    Welcome to File Moth    \x1b[0m");  // println!("\x1b[47;30m⊛   Welcome to File Warp!  ⊛\x1b[0m");
    //println!("┌──────────────────────────┐");
    println!("│You have \x1b[1;95m{}\x1b[0m projects.    {}│", count, spaces); // \x1b[38;5;239mv1.0\x1b[0m
    println!("├──────────────────────────┤");
    println!("│\x1b[1;92m1:\x1b[0m safe all projects      │");
    println!("│\x1b[1;92m2:\x1b[0m safe a project         │");
    println!("│\x1b[1;92m3:\x1b[0m edit projects          │");
    println!("│\x1b[1;92m4:\x1b[0m view projects          │");
    println!("│\x1b[1;92m5:\x1b[0m Settings               │");
    println!("│\x1b[1;92mq:\x1b[0m quit application       │");
    println!("└──────────────────────────┘");

    //let input = print_input("hahahahahahp");
    //println!("{}", input);
}

pub fn menu_print () {
    println!("\n");
    println!("\x1b[107;30mBack to the main menu.\x1b[0m");
    println!("\x1b[1;92m1:\x1b[0m safe all projects      ");
    println!("\x1b[1;92m2:\x1b[0m safe a project         ");
    println!("\x1b[1;92m3:\x1b[0m edit projects          ");
    println!("\x1b[1;92m4:\x1b[0m view projects          ");
    println!("\x1b[1;92m5:\x1b[0m Settings               ");
    println!("\x1b[1;92mq:\x1b[0m quit application       ");

}

fn system_message (message: &str) {

    // I think this can only be done once -> otherwise error
    // tell macOS which app "owns" the notification first
    let bundle = get_bundle_identifier_or_default("terminal"); // or "finder", your own bundle id, ...
    if let Err(error) = set_application(&bundle){
        println!("error: {}", error);
    }

    // Builds and sends a System Notification
    // When Terminal is in Scope (aka you have the terminal-window in focus) the message won't be shown
    let mut note = Notification::new();
    let note = note
        .content_image("/Users/lengather/Pictures/Bones/hello-fuckin-kitty-big-1.png")
        .title("File Moth")
        .message(message);

    if let Err(error) = note.send() {
        println!("Notification couldnt be send: {}", error);
    }
}

// Checks whether or not the config file exists.
pub fn file_check(){

    let path_folder: PathBuf = dir::home_dir().expect("home directory not found").join(".config/file_moth");
    let path_config: PathBuf = dir::home_dir().expect("home directory not found").join(".config/file_moth/file_moth_config.toml");

    println!("++ Path: {}", &path_config.display());

    //let path = "projects_test.toml";
    //let path_folder = Path::new("~/.config/file_moth");
    //let path_config = Path::new("~/.config/file_moth/file_moth_config.toml");
    //let path_config_check = Path::new(path_config);

    //println!("test");
    if path_folder.exists(){
        //If the log file is found the amount of projects needs to be checked.
        println!{"++ folder exists."};
    } else {
        // If the log file isn't found a new one is created.
        println!("config folder not found.");
        if let Err(e) = fs::create_dir(path_folder) {
            println!("error while creating folder: {}", e);
        }
    }

    if path_config.exists() {
        println!("++ config file exists");
    }else {
        println!("config file doesn't exist.");

        if let Err(error) = File::create(&path_config) {
            println!("Error while creating config file: {}", error);
        }
    }

    // If the config isn't correct Err is returned and a new one is created
    match open_toml() {
        Ok(_) => {
            //string to int

        },
        Err(()) => {
            println!("New config file needs to be build.");
            write_new_config(&path_config);
            println!("New config file has been created.");
        },
    };
}

// bundles and compresses files
fn bundle_compress (dest: &Vec<String>, origin: &String) -> Option<String>{

    let mut origin_path;

    match dest[2].as_str() {
        "zip" => {
            origin_path = String::from(origin);
            origin_path.push_str(".zip");

            match Command::new("zip"). args(["-r", &origin_path.as_str(), origin]).output() {
                Err(e) => {

                    println!("Error while calling zip. Check if zip is available.");
                    println!("error: {}", e);
                }
                Ok(out) => {

                    if out.status.success() {
                        //println!("++ zip has been succsesfully created: {}", String::from_utf8_lossy(&out.stdout));
                    } else {
                        eprintln!("\x1b[1mError during zip creation:\x1b[0m {}", String::from_utf8_lossy(&out.stderr));
                    }
                }
            }

            Some(origin_path)
        }
        "tar" => {
            // tar -czf furz.tar furz
            origin_path = String::from(origin);
            origin_path.push_str(".tar");

            match Command::new("tar"). args(["-czf", &origin_path.as_str(), origin]).output() {
                Err(e) => {
                    println!("Error while calling tar. Check if tar is available.");
                    println!("error: {}", e);
                }
                Ok(out) => {
                    if out.status.success() {
                        println!("++ tar has been succsesfully created: {}", String::from_utf8_lossy(&out.stdout));
                    } else {
                        eprintln!("\x1b[1mError during tar creation:\x1b[0m {}", String::from_utf8_lossy(&out.stderr));
                    }
                }
            }

            Some(origin_path)
        }
        "tar-gzip" => {
            origin_path = String::from(origin);
            origin_path.push_str(".tgz");

            match Command::new("tar"). args(["-czf", &origin_path.as_str(), origin]).output() {
                Err(e) => {
                    println!("Error while calling tar. Check if tar is available.");
                    println!("error: {}", e);
                }
                Ok(out) => {
                    if out.status.success() {
                        println!("++ tgz has been succsesfully created: {}", String::from_utf8_lossy(&out.stdout));
                    } else {
                        eprintln!("\x1b[1mError during tar creation:\x1b[0m {}", String::from_utf8_lossy(&out.stderr));
                    }
                }
            }

            Some(origin_path)
        }
        _ => {
            println!("config file is corrupted.");
            None
        }
    }
}

// Function that safes the projects
pub fn safe_dir (all: &str, config: Config){

    // check if there are any projects
    if config.general.amount == 0 {
        println!("You don't have any projects.");
        return;
    }

    // Overview of all projects
    if all == "n" {
        //println!("\n\x1b[107;30mProject Overview:\x1b[0m");
        println!("\n");
        view_projects("n");
        //println!("\n");
        println!("Type [b] if you want to go back.");
    }


    // retrieving the amount of projects
    // Array entrys can only be selected with usize numbers but amount is i64 -> its converted to usize
    let amount = match usize::try_from(config.general.amount) {
        Ok(v) => v,
        Err(_) => {
            println!("error during conversion of 'amount'");
            return
        },
    };

    // The index of the project in the array
    let mut index = 0;
    let count;

    // when user chooses to only safe one project
    if all == "n" {
        let mut input = print_input("\x1b[38;5;50;1mEnter the Number of the project you want to safe:\x1b[0m");

        loop {

            if input == "b" {
                return;
            }

            match input.parse::<usize>() {
                Ok(num) => {


                    // checking if the number is bigger then amount or smaller then one -> invalid
                    if num > amount || num < 1 {
                        input = print_input("\x1b[1mEnter valid input!\x1b[0m");
                        continue;
                    }
                     // index is calculated
                    index = num - 1;
                    count = index;
                    break;
                }
                Err(_) => {
                    input = print_input("\x1b[1mEnter valid input!\x1b[0m");
                    continue;
                }
            };
        }
    } else {
        count = amount - 1;
    }


    // The safe ------------------------------------------------
    while index <= count {
        // Looping through all destinations of a single project
        for dest in &config.project.destination[index] {

            // Depending on what number is in the mode-entry (dest[0]) of the Vector the right safe is selected
            // dest = ["1", "/user/path/", "zip"]
            match dest[0].as_str() {
                "1" => {
                    // normal safe
                    // copy via rsync
                    let _ = match Command::new("rsync").args(["-a", "--delete", config.project.origin[index].as_str(), dest[1].as_str()]).output() {
                        Ok(out) => out,
                        Err(e) => {
                            eprintln!("failed to run rsync: {e}");
                            return;
                        }
                    };
                    //println!("{}", out.status);
                }
                "2" => {
                    // Tailscale
                    //println!("++ safe via tailscale");

                    // the path of the bundled directory will be safed in there
                    let origin_path;

                    // bundling / compression
                    match bundle_compress(dest, &config.project.origin[index]) {
                        Some(path) => {
                            origin_path = path;
                        }
                        None => {
                            return;
                        }
                    }

                    // destination string needs to be created before because the ":" needs to be appended just after the dest like: "my-device:"
                    let mut destination = String::from(&dest[1]);
                    destination.push(':');

                    //spinner from the "indicatif" crate
                    let spinner = ProgressBar::new_spinner();
                    spinner.enable_steady_tick(Duration::from_millis(100));

                    match Command::new("tailscale").args(["file", "cp", &origin_path, destination.as_str()]).output() {
                        Err(e) => {
                            spinner.finish();
                            eprintln!("error while calling calling tailscale: {}", e);
                        }
                        Ok(out) => {
                            spinner.finish();
                            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();

                            //println!("++ status:{}", out.status);
                            if out.status.success() {
                                println!("++ tailscale call succsesfull: {}", stdout);
                            } else {
                                eprintln!("\x1b[1mTailscale couldn't safe the project:\x1b[0m {}", stderr);
                                return;
                            }


                            //println!("++ stdout length: {}", stdout.len());
                            //let message: Vec<&str> = stdout.split_whitespace().collect();
                        }
                    }
                }
                "3" => {
                    // SSH
                    println!("++ safe via SSH");
                }
                _ => {
                    println!("corrupted config file.");
                    return;
                }
            }
        }

        index = index + 1;
    }

    // loop through the destinations in the array

    if all == "n" {
        println!("The project has been safed.");
        system_message("Project safed");
    } else {
        println!("All projects have been safed.");
    }
}

fn read_toml(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("can't read file: {e}"))?;
    toml::from_str(&content)
        .map_err(|e| format!("invalid TOML: {e}"))
}

//open up the project toml
pub fn open_toml()-> Result<Config,()> {

    let path_config: PathBuf = dir::home_dir().expect("home directory not found").join(".config/file_moth/file_moth_config.toml");

    let path_str;
    match path_config.to_str() {
        Some(s) => {
            path_str = s;
        }
        None => {
            eprintln!("Path is not valid UTF-8");
            return Err(())
        },
    }

    match read_toml(&path_str) {
        Ok(config) => {

            //println!("{config:#?}");
            Ok(config)

        },
        Err(e) => {
            eprintln!("config error: {e}");
            Err(())
        },
    }
}

// Pause until enter is pressed
pub fn pause() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read input");
}

// Vectos are transformed into arrays
// With <T: Into<Value>> both Strings and i64 can be passed  to the function
fn vec_to_array<T: Into<Value>>(vec: Vec<T>) -> Array{
    let mut arr = Array::new();
    for x in vec {
        arr.push(x);
    }

    return arr;
}

fn write_new_config (path: &PathBuf) {

    //let path = "projects_test.toml";
    let mut doc = DocumentMut::new();

    // empty arrays which can be filled afterwards
    let empty_array = || Item::Value(Value::Array(Array::new()));

    // [General] Table
    let mut table_general = Table::new();
    table_general.insert("amount", 0.into());
    table_general.insert("daytime", empty_array());
    table_general.insert("weekday", empty_array());
    table_general.insert("monthday", empty_array());

    // [Settings]
    let mut table_settings = Table::new();
    table_settings.insert("notifications", 1.into());
    table_settings.insert("manual_input", 0.into());

    // [Project]
    let mut table_project = Table::new();
    table_project.insert("name", empty_array());
    table_project.insert("origin", empty_array());
    table_project.insert("destination", empty_array());
    table_project.insert("daemon", empty_array());

    // The tables are added to the Document
    doc["general"] = Item::Table(table_general);
    doc["settings"] = Item::Table(table_settings);
    doc["project"] = Item::Table(table_project);

    // The Document is written into the toml
    if let Err(error) = std::fs::write(path, doc.to_string()) {
        println!("Config File couldnt be renewt: {}", error);
    }
}

fn change_toml (proj: Project, amount: i64) -> Result<(), Box<dyn Error>>{
    //let path = "~/.config/file_moth/file_moth_config.toml";
    let path_config: PathBuf = dir::home_dir().expect("home directory not found").join(".config/file_moth/file_moth_config.toml");

    let path_str;
    match path_config.to_str() {
        Some(s) => {
            path_str = s;
        }
        None => {
            eprintln!("Path is not valid UTF-8");
            return Err("Error while reading path".into());
        },
    }
    // 1. Read the raw file as a string
    let content = std::fs::read_to_string(&path_str).map_err(|e| format!("read error: {e}"))?;

    // 2. Parse into a mutable document
    let mut doc = content.parse::<DocumentMut>().map_err(|e| format!("parse error: {e}"))?;

    //println!("++ toml has been read");

    // turning Vec into Array
    // Toml can only take Arrays

    let mut arr = Array::new();

    // proj.destination = [[["1", "/user/path/"], ["2", "device-in-tailnet"]], ...]
    // destinations = [["1", "/user/path/"], ["2", "device-in-tailnet"]]
    for destinations in proj.destination {
        // dest = ["1", "/user/path/"]
        let mut arr_sub = Array::new();
        for dest in destinations {
            arr_sub.push(vec_to_array(dest));
        }

        arr.push(arr_sub);
    }


    // The struct is serealized to toml table
    //let mut doc = DocumentMut::new();

    // creating table
    let mut new_table = Table::new();
    new_table.insert("name", vec_to_array(proj.name).into());
    new_table.insert("origin", vec_to_array(proj.origin).into());
    new_table.insert("destination", arr.into());
    new_table.insert("daemon", vec_to_array(proj.daemon).into());

    //println!("++ struct to table");

    // New Table is inserted into the document
    doc["project"] = Item::Table(new_table);

    // Amount is changed + 1
    doc["general"]["amount"] = toml_edit::value(amount);

    //println!("++ create new doc");
    std::fs::write(&path_str, doc.to_string())?;

    //println!("++ wrtiten to toml");

    Ok(())
}

// Function for using the directory picker
pub fn pick_folder_ext () -> Option<String> {

    // The exectuable which opens the picker is called
    // It prints the path to stdout which is collected by ".output"
    let out = match Command::new("/Users/lengather/Code-Lokal/file_pick/target/debug/file_pick").output() {
            Ok(out) => out,
            Err(e) => {
                eprintln!("failed to pick folder: {e}");
                return None;
            }
        };

    // check if the picking worked
    if !out.status.success() {
        eprintln!(
            "child_program failed with status {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
        return None;
    }

    // out is converted from Option to a String
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();

    if path.is_empty() {
        None // user cancelled, or program printed nothing
    } else {
        Some(path)
    }
}



// === ADDING PROJECT ===

fn add_project(mut config: Config, manual: &str){

    // I cloud also just write into the config thats passed
    let mut new_proj = config.project.clone();

    // 1
    let input_string = print_input("[1/4] \x1b[38;5;50;1mPlease enter a projectname:\x1b[0m");
    new_proj.name.push(input_string);

    // This one is gonna be reused for destination
    let new_path;

    // 2
    // ORIGIN ---------------------------

    // If manual is turned off in the settings
    if manual == "n" {
        println!("[2/4] \x1b[38;5;50;1mPlease select the directory you want to safe.\x1b[0m \x1b[1;37m[Press Enter]\x1b[0m");
        pause();

        // HIER JZ ANDERES PROGRAMM RUFEN

        let folder = pick_folder_ext();

        new_path = match folder {
            Some(path) => {
                println!("Selected origin: {}", path);
                path
            },
            None => {
                println!("New project cancled. Back to Menu.");
                return;
            },
        };
    } else {
        // Manual input of origin path
        let mut path_str = print_input("[2/4] \x1b[38;5;50;1mPlease enter the full path to the directory you want to safe:\x1b[0m");

        loop {

            let path = Path::new(&path_str);
            match path.try_exists() {
                Ok(result) => {
                    if result == true {
                        break;
                    } else {
                        path_str = print_input("[2/4] \x1b[38;5;50;1mThe path you entered doesn't exist. Please enter another path:\x1b[0m");
                    }
                }
                Err(_)=> {
                    path_str = print_input("[2/4] \x1b[38;5;50;1mThe path you entered doesn't exist. Please enter another path:\x1b[0m");
                }
            }
        }

        new_path = path_str;
    }


    new_proj.origin.push(new_path);



    // 3
    // Destination -----------------------------------------------------------------------------------

    // Destination is gonna be safed in there in all cases
    let mut dest_vec = Vec::new();

    println!("\n[3/4] \x1b[38;5;50;1mDestination\x1b[0m");
    println!("\x1b[1mThere are multiple options where and how to safe your origin-directory. Select one:\x1b[0m");
    println!("  \x1b[1m1.\x1b[0m Safe to directory on your device or your network");
    println!("  \x1b[1m2.\x1b[0m Safe via Tailscale (using Taildrop)");
    println!("  \x1b[1m3.\x1b[0m Safe via SSH");

    let mut choice;

    // Loop that selects chosen destination selection
    loop {

        choice = print_input("");

        match choice.trim() {
            "1" => {
                match com::setup_destination(manual, &new_proj) {
                    Some(dest) => {

                        dest_vec.push(dest);
                    }
                    None => {
                        return;
                    }
                }
                //break;
            }
            "2" => {
                match com::setup_tailscale_ssh(2) {
                    Some(dest) => {
                        //println!("++ tailscale worked");
                        //println!("++ mode: {}, dest: {}", dest[0], dest[1]);

                        dest_vec.push(dest);

                    }
                    None => {
                        return;
                    }
                }
                //break;
            }
            "3" => {
                match com::setup_tailscale_ssh(3) {
                    Some(_) => {
                        println!("++ SSH worked");
                    }
                    None => {
                        return;
                    }
                }
                //break;
            }
            _ => {
            choice = print_input("\x1b[38;5;50;1mEnter valid input!\x1b[0m")
            }
        }

        //let mut choice = print_input("[3/4] \x1b[38;5;50;1mDo you want to add an additional Destination? [y/n]\x1b[0m");
        let mut choice = print_input("\x1b[1mDo you want to add an additional Destination? [y/n]\x1b[0m");

        loop {
            match choice.trim() {
                "y" => {
                    //println!("Adding additional Destionation");
                    println!("  \x1b[1m1.\x1b[0m Safe to directory on your device or your network");
                    println!("  \x1b[1m2.\x1b[0m Safe via Tailscale (using Taildrop)");
                    println!("  \x1b[1m3.\x1b[0m Safe via SSH");
                    break;
                },
                "n" => {
                    break;
                }
                _ => {
                    //println!("Enter valid Input!");
                    choice = print_input("[3/4] \x1b[38;5;50;1mEnter valid input! [y/n]\x1b[0m");
                }
            }
        }

        if choice == "n" {
            break;
        }
    }

    // If 1 (regular destination) was selected
    if choice == "1"{



    }




    new_proj.destination.push(dest_vec);

    // 4
    // Dameon mode --------------------------------------

    let mut choice = print_input("[4/4] \x1b[38;5;50;1mDo you want to enable auto-safe? [y/n]\x1b[0m");
    //println!("received = {}", choice);
    loop {
        match choice.trim() {
            "y" => {
                // Choices for the auto-safe/daemon mode
                println!("\x1b[1mThere are multiple options how to use auto-safe. Enter the number of the mode you want to use.\x1b[0m");
                println!("  \x1b[1m1.\x1b[0m Safe when the directory is modified");
                println!("  \x1b[1m2.\x1b[0m Safe once a day (time can be set)");
                println!("  \x1b[1m3.\x1b[0m Safe once a week (weekday and time can be set)");
                println!("  \x1b[1m4.\x1b[0m Safe once a month (day and time can be set)");
                println!("  \x1b[1m5.\x1b[0m Don't use auto-safe");

                loop {

                    let mode = print_input("");

                    match mode.trim() {
                        "1" => {
                            println!("mode 1");
                            break;
                        }
                        "2" => {
                            println!("mode 2");
                            break;
                        }
                        "3" => {
                            println!("mode 3");
                            break;
                        }
                        "4" => {
                            println!("mode 4");
                            break;
                        }
                        "5" => {
                            break;
                        }
                        _ => {
                            println!("\x1b[1mEnter valid input!\x1b[0m");
                        }
                    }
                }


                new_proj.daemon.push(1);
                break;
            },
            "n" => {
                new_proj.daemon.push(0);
                break;
            }
            _ => {
                //println!("Enter valid Input!");
                choice = print_input("\x1b[1mEnter valid input! [y/n]\x1b[0m");
            }
        }
    }

    // Config file is changed -----------------------------------------------------

    config.general.amount = config.general.amount + 1;

    let proj_pass = new_proj.clone();

    // the finished Project and the new amount are passed to adding function
    if let Err(error) = change_toml(proj_pass, config.general.amount) {
        println!("error while adding: {}", error);
    }

    // Overview of the new project -----------------------------------------------

    // the position if the new project entrys in the Vectors
    // used to print the new entrys
    let position = &new_proj.name.len() - 1;

    println!("\n");
    println!("\x1b[107;30mA new project has been created.\x1b[0m");
    println!("name:  \x1b[1;95m{}\x1b[0m", &new_proj.name[position]);
    println!("origin:  \x1b[1;95m{}\x1b[0m", &new_proj.origin[position]);
    println!("selected destinations:");
    //println!("destination: \x1b[1;95m{}\x1b[0m", new_proj.destination);
    let mut count = 1;
    for dest in &new_proj.destination[position] {

        match dest[0].as_str() {
            "1" => println!("{}]  \x1b[1;95m{}\x1b[0m",count , dest[1]), // normal destination
            "2" => println!("{}]  \x1b[1;95m{}\x1b[0m (Tailscale + {})",count , dest[1], dest[2]), // Tailscale
            "3" => println!("{}]  \x1b[1;95m{}\x1b[0m (SSH)",count , dest[1]), // SSH
            _ => {
                println!("error while finishing up project");
                return;
            }
        }

        count = count + 1;
    }
}

// === DELETE PROJECT ===

fn delete_project (mut config: Config){

    if config.general.amount == 0 {
        println!("You don't have any projects.");
        return;
    }

    println!("\n\x1b[107;30mProject Overview:\x1b[0m");
    view_projects("n");
    println!("\n");
    println!("Type [b] if you want to go back.");

    // Array entrys can only be selected with usize numbers but amount is i64 -> its converted to usize
    let mut amount = match usize::try_from(config.general.amount) {
        Ok(v) => v,
        Err(_) => {
            println!("error during conversion of 'amount'");
            return
        },
    };

    // The index of the project in the array
    let index;
    let mut choice_all = "n";
    let mut input = print_input("\x1b[93;1mEnter the Number of the project you want to delete. If you want to delte all projects enter [all].\x1b[0m");

    loop {

        if input == "b" {
            return;
        }

        if input == "all" {
            choice_all = "y";
            index = amount - 1;
            break;
        }

        match input.parse::<usize>() {
            Ok(num) => {


                // checking if the number is valid
                if num > amount || num < 1 {
                    input = print_input("\x1b[93;1mEnter valid input!\x1b[0m");
                    continue;
                }
                 // index is calculated
                index = num - 1;
                break;
            }
            Err(_) => {
                input = print_input("\x1b[93;1mEnter valid input!\x1b[0m");
                continue;
            }
        };
    }

    // Case all delete and normal delete
    if choice_all == "y" {
        input = print_input("\x1b[93;1mAre you sure you want to delete all projects? [y/n]\x1b[0m");
    } else {
        println!("You have selected project {}.", index + 1);
        input = print_input("\x1b[93;1mAre you sure you want to delete this project? [y/n]\x1b[0m");
    }

    loop {
        match input.trim() {
            "y" => {

                if choice_all == "y" {
                    println!("++ first index = {}", index);
                    // Removing all projects
                    // They need to be removed in reversed order. Otherwise the indexes wont exist no more after half the Vec is gone.
                    for i in (0..=index).rev() {
                        println!("++ index = {}", i);

                        config.project.name.remove(i);
                        config.project.origin.remove(i);
                        config.project.destination.remove(i);
                        config.project.daemon.remove(i);
                    }
                    println!("All projects have been removed");
                    amount = 0;
                    break;
                } else {
                    // Removing project at index
                    config.project.name.remove(index);
                    config.project.origin.remove(index);
                    config.project.destination.remove(index);
                    config.project.daemon.remove(index);

                    println!("The project has been removed.");
                    amount = amount - 1;
                    break;
                }

            },
            "n" => {
                return;
            }
            _ => {
                //println!("Enter valid Input!");
                input = print_input("\x1b[93;1mEnter valid input! [y/n]\x1b[0m");
            }
        }
    }

    let amount_int: i64 = amount as i64;
    if let Err(error) = change_toml(config.project, amount_int) {
        println!("Erorr while changing config file: {}", error);
    }

}

// === ADDING PROJECT OPTION MENU ===

pub fn edit_projects(){

    //Options for project editing are printed
    println!("  \x1b[1;94ma:\x1b[0m add new project");
    println!("  \x1b[1;94md:\x1b[0m delete existing project");
    println!("  \x1b[1;94mb:\x1b[0m go back");

    let input = print_input("");

    match input.trim(){
        "a" => {

            match open_toml() {
                Ok(config) => {
                    //string to int
                    add_project(config, "n");

                },
                Err(()) => println!("couldnt add"),
            }
        },
        "d" => {
            match open_toml() {
                Ok(config) => {
                    //string to int
                    delete_project(config);

                },
                Err(()) => println!("Error while opening config file."),
            }
        }
        "b" => {
            //println!{"back to main menu"};

        },
        _ => {
            println!("bruh");
        },
    }
}

// All the projectnames are printed
pub fn view_projects (addiotional: &str) {

    //let mut count = 0;
    match open_toml() {
        Ok(config) => {

            if config.general.amount == 0 {
                println!("You don't have any projects yet.");
                return;
            }

            // Array entrys can only be selected with usize numbers but amount is i64 -> its converted to usize
            let amount = match usize::try_from(config.general.amount) {
                Ok(v) => v,
                Err(_) => {
                    println!("error during conversion of 'amount'");
                    return
                },
            };

            println!("\x1b[107;30mProject Overview\x1b[0m");

            for proj in 0 .. amount {
                println!("\x1b[1;95m{}]\x1b[0m {}", proj + 1, config.project.name[proj]);

                let mut count = 1;
                // In case only the names are wanted ("n")the rest is not printed
                if addiotional == "y" {
                    println!("  \x1b[1morigin:\x1b[0m ");
                    println!("     {}", config.project.origin[proj]);
                    println!("  \x1b[1mdestinations:\x1b[0m");

                    for dest in &config.project.destination[proj] {
                        //println!("  {}] {}", count, dest[1]);
                        match dest[0].as_str() {
                            "1" => println!("{}]  {}",count , dest[1]), // normal destination
                            "2" => println!("{}]  {} (Tailscale + {})",count , dest[1], dest[2]), // Tailscale
                            "3" => println!("{}]  {} (SSH)",count , dest[1]), // SSH
                            _ => {
                                println!("error while finishing up project");
                                return;
                            }
                        }
                        count = count + 1;
                    }
                    println!("\n");
                }


                //count = count + 1;
            }

        },
        Err(()) => println!("error while opening config file."),
    }
}

pub fn settings () {
    println!("\n  \x1b[107;30mEnter the number of the setting you want to adjust.\x1b[0m");
    println!("  \x1b[1;94mn:\x1b[0m notifications  [\x1b[92mon\x1b[0m/off]");
    println!("  \x1b[1;94mm:\x1b[0m manual input   [on/\x1b[95moff\x1b[0m]");
    println!("  \x1b[1;94mb:\x1b[0m go back");
}

pub fn art (){

    let moth_3 = "\x1b[38;5;55m
                            ü.      _.
                             '\\-o ,/''
            ___,x.Y----D-----''.b.'._----....b___,_.
         ,-''    ,.,b  .     \\'i'',o'` .    ,__4'''x\\\\.
        a'   `'  |. |  `_  ,,  ''A'     _.  |'  \\      '(.
        \\.        'g'    ,-'    `     `.   '-v-'    '  ,|
          ' b. .       ,''      L.      `.            S'
             'B.'i'  ,Y,Y'@'. `/'`-  /F-\\'\\.   ,- ,_o''
                '`-''  |_ _'        |_  |  `...---'
                   \\.  Free             Earth  |-
                    `\\_        ,,L         ,,
                      ''f--'r'o' ''`---u--''
    \x1b[0m";


println!("{}", moth_3);
//println!("Fuck da System");

}
