use std::env;
use std::io::Result;
use std::io;
use winreg::enums::KEY_READ;
use winreg::RegKey;

pub fn get_computer_name() -> Result<String> {
    match env::var("COMPUTERNAME") {
        Ok(name) => Ok(name),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Computer name not found")),
    }
}

pub fn list_installed_apps(uninstall_key: &RegKey) -> io::Result<Vec<String>> {
    let mut apps = Vec::new();

    for subkey_name in uninstall_key.enum_keys() {
        let subkey_name = subkey_name?;
        let subkey = uninstall_key.open_subkey_with_flags(&subkey_name, KEY_READ);

        if let Ok(subkey) = subkey {
            // Попытаемся получить значение DisplayName (имя приложения)
            if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                apps.push(display_name);
            }
        }
    }

    Ok(apps)
}
