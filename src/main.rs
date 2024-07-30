use crossterm::{
    cursor,
    style::{Print, PrintStyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use crossterm::{
    event::{self, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;
use std::{env::args, fs::File, io::read_to_string};
use std::{
    io::{self, Write},
    thread::sleep,
};

fn main() {
    let presen =
        read_to_string(File::open(args().collect::<Vec<String>>()[1].clone()).unwrap()).unwrap();
    slide_show(presen)
}

fn wait_keypress() {
    let _ = enable_raw_mode();
    loop {
        if event::poll(Duration::from_secs(1)).unwrap_or(false) {
            if let event::Event::Key(KeyEvent { .. }) = event::read().unwrap() {
                break;
            }
        }
    }
    let _ = disable_raw_mode();
}

fn slide_show(presen: String) {
    for slide in presen.split("\n\n") {
        draw_slide(slide.to_string()).unwrap();
    }
    println!("");
}

fn draw_slide(slide: String) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;

    let mut line_count = 1;
    for text in slide.split("\n") {
        let text: String = text.to_owned() + "\n";
        show_text(text.trim().to_string(), line_count)?;
        line_count += 1;
        wait_keypress();
    }

    Ok(())
}
fn show_text(text: String, line_count: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (width, _) = terminal::size()?;
    let terminal_width = width as usize;
    let text_length = text.len();
    let position = (terminal_width - text_length) / 2;

    if text.contains("->") {
        stdout.queue(cursor::MoveTo(position as u16, line_count))?;
        let (before, after) = {
            let splited = text.split("->").collect::<Vec<&str>>();
            (
                splited[0].trim().to_string(),
                splited[1..splited.len()].join("->").trim().to_string(),
            )
        };
        show_text(before, line_count)?;
        wait_keypress();
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        show_text(after, line_count)?;
    } else if text.starts_with(">>") {
        for i in position - 20..position {
            stdout.queue(cursor::MoveTo(i as u16, line_count))?;
            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
            stdout.queue(Print(text.replacen(">>", "", 1).clone()))?;
            stdout.flush()?;
            sleep(Duration::from_secs_f64(0.01));
        }
    } else if text.starts_with("#") {
        stdout.queue(cursor::MoveTo(position as u16, line_count))?;
        stdout.queue(PrintStyledContent(
            text.replacen("#", "", 1)
                .trim()
                .attribute(crossterm::style::Attribute::Bold)
                .attribute(crossterm::style::Attribute::Underlined),
        ))?;
    } else {
        stdout.queue(cursor::MoveTo(position as u16, line_count))?;
        stdout.queue(Print(text))?;
    }
    stdout.flush()?;
    Ok(())
}
