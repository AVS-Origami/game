use std::fs;
use std::process;

use std::error::Error;

use native_dialog::{MessageDialog, MessageType};

 pub enum SType<F, I, S> {
    Float(F),
    Int(I),
    String(S),
}

fn get_setting(setting: &str, s_type: &SType<f32, i32, String>) -> Result<SType<f32, i32, String>, Box<dyn Error>> {
    let settings_file = fs::read_to_string(String::from("settings.cfg"))?;

    let mut result_str = String::new();
    let mut result_found = false;

    for line in settings_file.lines() {
        if line.contains(setting) {
            for c in line.chars() {
                if c == '#' {
                    break;
                } else if result_found {
                    result_str.push(c);
                } else if c == '=' {
                    result_found = true;
                }
            }

            if result_str.len() > 0 {
                result_str.retain(|c| !c.is_whitespace());

                let result = match s_type {
                    SType::Float(_) => SType::Float(result_str.parse::<f32>().unwrap()),
                    SType::Int(_) => SType::Int(result_str.parse::<i32>().unwrap()),
                    SType::String(_) => SType::String(result_str),
                };

                
                return Ok(result);
            }
        }
    }

    return Err("could not locate setting ".to_owned() + setting)?;
}

pub fn fetch_setting(setting: &str, s_type: &SType<f32, i32, String>) -> SType<f32, i32, String> {
    match get_setting(setting, s_type) {
        Ok(t) => return t,
        Err(e) => {
            let _ = MessageDialog::new().set_title("game").set_text(&format!("Error while loading settings: {}.", e)).set_type(MessageType::Error).show_alert();
            eprintln!("Error while loading settings: {}.", e);
            process::exit(1)
        }
    }
}