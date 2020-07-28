use std::process::Command;

fn main() {
    // gio mount -d /dev/sda3
    let mut terminal = Command::new("gio");
    terminal.arg("mount");
    terminal.arg("-d");
    terminal.arg("/dev/sda3");

    //Execute the command
    match terminal.output() {
        Ok(_o) => {
            println!("Success! Mounted the HDD!");
        },
        Err(e) => {
            println!("There was an error {}", e);
        }
    }
}
