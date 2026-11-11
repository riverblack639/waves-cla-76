//! Windows registry read helpers for Waves Cla 76.

pub const APP_KEY: &str = "Software\\Waves\\Waves Cla 76";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Waves Cla 76";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Waves Cla 76") },
        RegValue { name: "Version",     value: String::from("v4.8.21") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Waves Cla 76") },
        RegValue { name: "Publisher",       value: String::from("Waves") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Waves Cla 76\\uninstall.exe\"") },
    ]
}
