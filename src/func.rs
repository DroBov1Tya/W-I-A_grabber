use std::env;
use std::io;
use std::io::Result;
use std::collections::HashMap;
use winreg::enums::KEY_READ;
use winreg::RegKey;


pub fn get_computer_name() -> Result<String> {
    match env::var("COMPUTERNAME") {
        Ok(name) => Ok(name),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Computer name not found")),
    }
}

pub fn list_installed_apps(uninstall_key: &RegKey) -> io::Result<HashMap<String, HashMap<String, Option<String>>>> {
    let mut installed_apps: HashMap<String, HashMap<String, Option<String>>> = HashMap::new();

    for subkey_name in uninstall_key.enum_keys().filter_map(|key| key.ok()) {

        if let Ok(subkey) = uninstall_key.open_subkey_with_flags(&subkey_name, KEY_READ) {

            if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                let display_version = subkey.get_value::<String, _>("DisplayVersion").ok();
                let publisher = subkey.get_value::<String, _>("Publisher").ok();
                let install_date = subkey.get_value::<String, _>("InstallDate").ok();
                let uninstall_string = subkey.get_value::<String, _>("UninstallString").ok();
                let install_location = subkey.get_value::<String, _>("InstallLocation").ok();
                let estimated_size = subkey.get_value::<u32, _>("EstimatedSize").ok().map(|s| s.to_string());

                let mut app_info = HashMap::new();
                app_info.insert("DisplayName".to_string(), Some(display_name));
                app_info.insert("DisplayVersion".to_string(), display_version);
                app_info.insert("Publisher".to_string(), publisher);
                app_info.insert("InstallDate".to_string(), install_date);
                app_info.insert("UninstallString".to_string(), uninstall_string);
                app_info.insert("InstallLocation".to_string(), install_location);
                app_info.insert("EstimatedSize".to_string(), estimated_size);

                installed_apps.insert(subkey_name, app_info);
            }
        }
    }
    Ok(installed_apps)
}