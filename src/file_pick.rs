use rfd;
//use std::io{self};

pub fn main() {
    // Finder is opend
    let mut new_path = String::new();
    let folder = rfd::FileDialog::new()
            .set_title("Pick a folder")
            .pick_folder();

    print("new_path");
}
