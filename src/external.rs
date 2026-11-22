// For functions that communicate with external services or files

pub mod com {

    use std::process::Command;
    use std::path::Path;
    use crate::utils::{self, print_input, pause, pick_folder_ext, Project};

    pub fn setup_destination(manual: &str, new_proj: &Project) -> Option<Vec<String>> {

        let mut new_path;

        if manual == "n" {
            println!("\x1b[1mPlease select the directory you want to safe the original to.\x1b[0m \x1b[1;37m[Press Enter]\x1b[0m");
            pause();
        }


        if manual == "n" {
            loop {

                let folder = pick_folder_ext();

                new_path = match folder {
                    Some(path) => {
                        println!("Selected destination: {}\n", path);
                        path
                    },
                    None => {
                        println!("New project cancled. Back to Menu.");
                        return None;
                    },
                };

                let index = new_proj.name.len();

                // checks if the destination path (new_path) and the origin are the same
                if new_path != new_proj.origin[index-1] {
                    break;
                }

                println!("\x1b[1mOrigin and destination can not be the same directory. Select a different directory!\x1b[0m \x1b[1;37m[Press Enter]\x1b[0m");
                pause();
            }
        } else {
            // Manual input of destination path
            let mut path_str = print_input("\x1b[1mPlease enter the full path to the directory you want to safe the original to:\x1b[0m");

            loop {

                let path = Path::new(&path_str);
                match path.try_exists() {
                    Ok(result) => {
                        if result == true {
                            break;
                        } else {
                            path_str = print_input("\x1b[1mThe path you entered doesn't exist. Please enter another path:\x1b[0m");
                        }
                    }
                    Err(_)=> {
                        path_str = print_input("\x1b[1mThe path you entered doesn't exist. Please enter another path:\x1b[0m");
                    }
                }
                // Hier muss auch noch getestet werden ob der pfad dem origin entspricht
            }

            new_path = path_str;
        }

        // Destination Vectors are created
        // first the Vector that contains the mode and the path is build like: ["1", "/user/someshit/"]
        let mut vec_sub = Vec::new();
        vec_sub.push("1".to_string());
        vec_sub.push(new_path);

        Some(vec_sub)
    }


    // Setup Tailscale ----------------------------------------------------------------

    pub fn setup_tailscale_ssh (mode: i16) -> Option<Vec<String>> {

        //println!("mode: {}", mode);

        if mode == 2 {
            // Checking if tailscale is available and working
            match Command::new("tailscale").arg("version").output() {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    println!("Tailscale is not installed: {}", e);
                    return None;
                }
                Err(e) => {
                    // installiert, aber anderes Problem (Permissions etc.)
                    println!("There seems to be a problem with tailscale: {}", e);
                    return None;
                }
                Ok(output) if output.status.success() => {
                    // da und lauffähig
                    //println!("Tailscale has been found and is working.");
                }
                Ok(_) => { /* da, aber versagt beim Start */
                    println!("Problem while starting tailscale.");
                    return None;
                }
            }

            // Looking for devices in the tailnet
            // +++ Hier muss noch der Fall behandelt werden, dass keine geräte gefunden werden!!!
            let out = match Command::new("tailscale").args(["file", "cp", "--targets"]).output() {
                Err(e) => {
                    println!("Error while looking for targets in the tailnet: {}", e);
                    //println!("There are no devices in your tailnet that you can send to. Add another device if you want to use this feature.");
                    return None;
                }
                Ok(output) => {

                    if output.status.success() {

                            //println!("{}", String::from_utf8_lossy(&output.stdout));
                            output
                    } else {
                            eprintln!("error: {}", String::from_utf8_lossy(&output.stderr));
                            println!("You might have to turn on tailscale.");
                            return None;
                    }
                }
            };

            // 'out' is a struct that contains stdout: Vec<u8> and stderr: Vec<u8>.
            // Vec<u8> is used because the stdout/err is safed as byte-stream that is UTF-8 encoded (u8 = one byte).
            // We want to take the stdout str and convert it to a String.

            let targets = String::from_utf8_lossy(&out.stdout);
            // Breaking up the target output in to its lines wich are safed in a Vector
            let target_lines: Vec<&str> = targets.lines().collect();

            // Printing the targets and prompt for a choice
            println!("\x1b[1mEnter the number of the device you want to safe the origin to. Enter [b] to go back.\x1b[0m");
            println!("Devices in your tailnet:");
            let mut count = 1;
            for x in &target_lines {
                println!("  \x1b[1m{}]\x1b[0m {}", count, x);
                count = count + 1;
            }

            //println!("b] don't use tailnet and go back");

            let mut input = utils::print_input("");

            // user wants to go back
            if input == "b" {
                return None;
            }

            let target;

            loop {
                match input.parse::<usize>() {
                    Ok(num) => {

                        // check if the number is smaller then zero or bigger then the amount of targets
                        if num < 1 || num > target_lines.len() {
                            input = print_input("\x1b[1mEnter valid input!\x1b[0m");
                            continue;
                        }

                        target = num;
                        break;
                    }
                    Err(_) => {
                        input = print_input("\x1b[1mEnter vaild input!\x1b[0m");
                    }
                };
            }

            // selecting the right line from all the lines in the Vector
            let target_line = target_lines[target-1];

            // Splitting up the line into its parts
            let target_parts: Vec<&str> = target_line.split_whitespace().collect();

            // Element 1 in the Vec is the name of the device -> will be used to call tailscale
            println!("\nSelected destination: {}\n", target_parts[1]);

            // creating the Vector that safes the device and the mode
            let mut dest = Vec::new();
            dest.push("2".to_string());
            dest.push(target_parts[1].to_string());

            // Asking what typa packaging or compression should be used
            println!("\x1b[1mTailscale can only send individual files. Choose how to bundle/compress the origin:\x1b[0m");
            println!("  \x1b[1m1.\x1b[0m Bundle and compress using ZIP (works on all paltforms)");
            println!("  \x1b[1m2.\x1b[0m Bundle using TAR (Linux & macOS)");
            println!("  \x1b[1m3.\x1b[0m Bundle and compress using TAR and GZIP (Linux & macOS)");
            input = print_input("");

            // input on the bundle / compression mode
            loop {
                match input.trim() {
                    "1" => {
                        // ZIP
                        dest.push("zip".to_string());
                        break;
                    }
                    "2" => {
                        // TAR
                        dest.push("tar".to_string());
                        break;
                    }
                    "3" => {
                        // TAR
                        dest.push("tar-gzip".to_string());
                        break;
                    }
                    _ => {
                        input = print_input("\x1b[1mEnter vaild input!\x1b[0m");
                    }
                };
            }

            Some(dest)

        }else {
            println!("setup SSH");
            //Some("shrank".to_string())
            None
        }
    }



}
