use std::process::Command;
use std::io;

fn main() -> io::Result<()> {
    println!("---------------------------------------------------");
    println!("Start nvim-update");
    let output_curl = Command::new("curl").arg("-LO").arg("https://github.com/neovim/neovim/releases/latest/download/nvim.appimage").output()?;

    if !output_curl.status.success() {
        eprintln!("Failed to download nvim.appimage");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to download nvim.appimage"));
    } else {
        println!("Successfull to download nvim.appimage");
    }

    let output_chmod = Command::new("chmod").arg("u+x").arg("nvim.appimage").output()?;

    if !output_chmod.status.success() {
        eprintln!("Failed to make nvim.appimage executable");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to make nvim.appimage executable"));
    } else {
        println!("Successfull to make nvim.appimage executable");
    }

    let output_sudo = Command::new("sudo").arg("mv").arg("nvim.appimage").arg("/usr/local/bin/nvim").output()?;

    if !output_sudo.status.success() {
        eprintln!("Failed to move nvim.appimage to /usr/local/bin/nvim");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to move nvim.appimage to /usr/local/bin/nvim"));
    } else {
        println!("Successfull to move nvim.appimage to /usr/local/bin/nvim");
    }

    let output_nvim_version = Command::new("nvim").arg("-v").output()?;

    if !output_nvim_version.status.success() {
        eprintln!("Failed to confirm neovim version");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to move nvim.appimage to /usr/local/bin/nvim"));
    } else {
        println!("Successfull to confirm neovim version");
        println!("{}", String::from_utf8_lossy(&output_nvim_version.stdout));
    }

    println!("End nvim-update");
    println!("---------------------------------------------------");

    Ok(())

}
