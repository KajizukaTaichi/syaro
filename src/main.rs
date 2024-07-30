use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal, QueueableCommand,
};
use std::io::{self, Write};
use std::{env::args, fs::File, io::read_to_string};

fn main() {
    let presen =
        read_to_string(File::open(args().collect::<Vec<String>>()[1].clone()).unwrap()).unwrap();
    slide_show(presen)
}

/// Get standard input
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn slide_show(presen: String) {
    for slide in presen.split("\n\n") {
        draw_slide(slide.to_string()).unwrap();
    }
}

fn draw_slide(slide: String) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;

    let mut line_count = 1;
    for text in slide.split("\n") {
        let text: String = text.to_owned() + "\n";

        let (width, _) = terminal::size()?;
        let terminal_width = width as usize;
        let text_length = text.len();
        let position = (terminal_width - text_length) / 2;

        stdout.queue(cursor::MoveTo(position as u16, line_count))?;
        if text.starts_with("#") {
            stdout.queue(PrintStyledContent(
                text.replacen("#", "", 1)
                    .attribute(crossterm::style::Attribute::Bold),
            ))?;
        } else {
            stdout.queue(PrintStyledContent(text.with(crossterm::style::Color::Grey)))?;
        }
        stdout.flush()?;
        line_count += 1;
        input("");
    }

    Ok(())
}
