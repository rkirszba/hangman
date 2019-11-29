pub mod config {

    pub enum Mode {
        Admin,
        Game
    }

    pub fn parse_args(args: Vec<String>) -> Result<Mode, &'static str>
    {
        if args.len() > 2 { return Err("Usage: cargo run [admin]"); }
        if args.len() == 2 {
            let arg = args[1].as_str();
            match arg {
                arg if arg != "admin" => return Err("Usage: cargo run [admin]"),
                _ => return Ok(Mode::Admin)
            }
        }
        Ok(Mode::Game)
    }
}
