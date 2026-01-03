use cica::color_space::{Hsv, Lab, Srgb, Xyz};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Color Space Converter (Interactive Mode)");
    println!("Type 'help' for available commands, 'quit' to exit.\n");
    print!("> ");
    stdout.flush().unwrap();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match parse_and_process(line) {
            Ok(None) => break, // quit command
            Ok(Some(output)) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        print!("> ");
        stdout.flush().unwrap();
    }

    println!("\nGoodbye!");
}

/// Parse command and process it
/// Returns:
/// - Ok(None) if quit command
/// - Ok(Some(output)) if successful conversion
/// - Err(message) if error
fn parse_and_process(line: &str) -> Result<Option<String>, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(Some(String::new()));
    }

    let command = parts[0].to_lowercase();

    match command.as_str() {
        "help" => Ok(Some(show_help())),
        "quit" | "exit" => Ok(None),
        "srgb" => {
            if parts.len() != 4 {
                return Err("Usage: srgb <r> <g> <b>".to_string());
            }
            let r = parse_f32(parts[1], "r")?;
            let g = parse_f32(parts[2], "g")?;
            let b = parse_f32(parts[3], "b")?;
            let srgb = Srgb::new(r, g, b);
            Ok(Some(format_all_color_spaces(srgb.into())))
        }
        "xyz" => {
            if parts.len() != 4 {
                return Err("Usage: xyz <x> <y> <z>".to_string());
            }
            let x = parse_f32(parts[1], "x")?;
            let y = parse_f32(parts[2], "y")?;
            let z = parse_f32(parts[3], "z")?;
            let xyz = Xyz::new(x, y, z);
            Ok(Some(format_all_color_spaces(xyz)))
        }
        "lab" => {
            if parts.len() != 4 {
                return Err("Usage: lab <l> <a> <b>".to_string());
            }
            let l = parse_f32(parts[1], "l")?;
            let a = parse_f32(parts[2], "a")?;
            let b = parse_f32(parts[3], "b")?;
            let lab = Lab::new(l, a, b);
            Ok(Some(format_all_color_spaces(lab.into())))
        }
        "hsv" => {
            if parts.len() != 4 {
                return Err("Usage: hsv <h> <s> <v>".to_string());
            }
            let h = parse_f32(parts[1], "h")?;
            let s = parse_f32(parts[2], "s")?;
            let v = parse_f32(parts[3], "v")?;
            let hsv = Hsv::new(h, s, v);
            Ok(Some(format_all_color_spaces(hsv.into())))
        }
        _ => Err(format!(
            "Unknown command: '{}'. Type 'help' for available commands.",
            command
        )),
    }
}

fn parse_f32(s: &str, name: &str) -> Result<f32, String> {
    s.parse::<f32>()
        .map_err(|_| format!("Invalid value for '{}': '{}'", name, s))
}

fn show_help() -> String {
    r#"Available commands:
  srgb <r> <g> <b>  Convert from sRGB (0.0-1.0)
  xyz <x> <y> <z>   Convert from CIE XYZ (0.0-1.0)
  lab <l> <a> <b>   Convert from CIE Lab (0-100, -128-127, -128-127)
  hsv <h> <s> <v>   Convert from HSV (h: 0-360, s: 0-1, v: 0-1)
  help              Show this help message
  quit / exit       Exit the program

Examples:
  > srgb 1.0 0.5 0.0
  > xyz 0.5 0.5 0.5
  > lab 50 25 -10
  > hsv 180 0.5 0.8"#
        .to_string()
}

fn format_all_color_spaces(xyz: Xyz) -> String {
    let srgb: Srgb = xyz.into();
    let lab: Lab = xyz.into();
    let hsv: Hsv = xyz.into();

    format!(
        "sRGB: ({:.6}, {:.6}, {:.6})\nXYZ:  ({:.6}, {:.6}, {:.6})\nLab:  ({:.6}, {:.6}, {:.6})\nHSV:  ({:.6}°, {:.6}, {:.6})",
        srgb.r, srgb.g, srgb.b, xyz.x, xyz.y, xyz.z, lab.l, lab.a, lab.b, hsv.h, hsv.s, hsv.v
    )
}
