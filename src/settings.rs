use std::fs;
use std::process;

use std::error::Error;

fn get_setting(setting: &str) -> Result<f32, Box<dyn Error>> {
    let settings_file = fs::read_to_string(String::from("settings.cfg"))?;

    let mut result_str = String::new();

    for line in settings_file.lines() {
        if line.contains(setting) {
            for c in line.chars() {
                if c == '#' {
                    break;
                } else if c.is_numeric() {
                    result_str.push(c);
                } else if c == '.' {
                    result_str.push(c);
                }
            }

            let result: f32 = result_str.parse::<f32>().unwrap();
            return Ok(result);
        }
    }

    return Err("could not locate setting")?;
}

pub fn fetch_setting(setting: &str) -> f32 {
    match get_setting("scale") {
        Ok(t) => return t,
        Err(e) => {
            eprintln!("Error while loading settings: {} '{}'. ", e, setting);
            process::exit(1);
        }
    }
}