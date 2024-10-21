use std::io;
use winreg::enums::*;
use winreg::RegKey;

mod func;
mod api;

fn main() -> io::Result<()> {
    let token: String = "".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let computer_name = func::get_computer_name();

    let uninstall = hklm.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall", KEY_READ)?;
    let big_bit =  func::list_installed_apps(&uninstall)?;

    let wow64_uninstall = hklm.open_subkey_with_flags(
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall", KEY_READ)?;
    let low_bit =  func::list_installed_apps(&wow64_uninstall)?;

    let _req = api::req(big_bit, low_bit, token, computer_name?);
    Ok(())
}

