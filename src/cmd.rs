///
/// # Command parser
///

pub enum Command {
    Paste,
    PasteNamed(String),
    Copy(String),
    Delete(String),
    Move(String, String),
}

pub fn parse(cmd: String) -> Result<Command, String> {
    let pieces: Vec<&str> = cmd.trim().split(' ').collect();
    match pieces.len() {
        3 => {
            match pieces[0] {
                "m" => Ok(Command::Move(pieces[1].to_string(), pieces[2].to_string())),
                _ => Err(String::from("Invalid 3 part command")),
            }
        },
        2 => {
            match pieces[0] {
                "c" => Ok(Command::Copy(pieces[1].to_string())),
                "p" => Ok(Command::PasteNamed(pieces[1].to_string())),
                "d" => Ok(Command::Delete(pieces[1].to_string())),
                _ => Err(String::from("Invalid 2 part command")),
            }
        },
        1 => {
            match pieces[0] {
                "p" | "v" => Ok(Command::Paste),
                _ => Err(String::from("Invalid 1 part command")),
            }
        }
        _ => Err(String::from("Invalid command"))
    }
}
