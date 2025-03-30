use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn converter(grammar: &str) -> io::Result<()> {
    let path = Path::new(grammar);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut rules_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let re = Regex::new(r"(\w+)\s*->\s*(.*)").unwrap();
    let terminal_re = Regex::new(r#""([^"]+)""#).unwrap();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() || line.trim().starts_with("//") {
            continue;
        }

        if let Some(caps) = re.captures(&line) {
            let lhs = caps[1].to_string();
            let rhs = caps[2]
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|rhs_str| {
                    rhs_str
                        .split_whitespace()
                        .map(|s| {
                            if terminal_re.is_match(s) {
                                terminal_re.replace(s, "$1").to_string()
                            } else {
                                s.to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();

            rules_map.entry(lhs).or_default().extend(rhs);
        }
    }

    let output_path: PathBuf = path.with_extension("json");
    let json = serde_json::to_string_pretty(&rules_map)?;

    std::fs::write(output_path, json)?;

    println!("Grammar has been parsed and saved.");

    Ok(())
}
