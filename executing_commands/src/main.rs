use std::process::Command;

fn main() {
    let output = Command::new("mullvad")
        .arg("relay")
        .arg("set")
        .arg("location")
        .arg("nl")
        .output()
        .expect("Failed switching relay");

    if output.status.success() {
        println!("Successfully changed relay");
    } else {
        println!("Failed to change relay");
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error);
    }
}
