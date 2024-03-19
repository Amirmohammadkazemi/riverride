use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};

fn main() -> std::io::Result<()> {

    let mut scr = stdout();
    scr.execute(Print("Welcome...!!!\n"))?;

    Ok(())
}