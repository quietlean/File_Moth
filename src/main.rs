//use std::io::{self};
//use std::{process};
mod external;
//use crate::external::com;

use crate::utils::{art, edit_projects, file_check, menu_print, menu_print_intro, safe_dir, view_projects, open_toml};


// Functions are in "utils.rs".
mod utils;

fn main() {

    file_check();   // checking if config file exists
    art();  // ascii art is printed

    menu_print_intro();  // menu is printed

    //utils::open_log();

    loop {

        let input = utils::print_input("");

        match input.trim() {
            "q" => break,
            "1" => {
                // safe all projects
                match open_toml() {
                    Ok(config) => {
                        //string to int
                        safe_dir("y" ,config);

                    },
                    Err(()) => println!("Error while opening config file."),
                }
            },
            "2" => {
                // safe a project
                match open_toml() {
                    Ok(config) => {
                        //string to int
                        safe_dir("n" ,config);

                    },
                    Err(()) => println!("Error while opening config file."),
                }
            },
            "3" => {
                edit_projects();
            },
            "4" => {

                view_projects("y"); // "y" means that the full overview is printed. "n" -> shortened overview
            },
            "5" => {
                utils::settings();
            }
            _ => println!("Choose valid input!"),
        }

        menu_print();
    }


}
