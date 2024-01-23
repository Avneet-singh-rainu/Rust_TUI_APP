use crossterm::event::read;
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, QueueableCommand};
use std::str::Bytes;
use std::{char, io};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////

struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

fn chat_window(stdout: &mut io::Stdout, chat: &[String], boundary: Rect) {
    let n = chat.len();
    let m = n.checked_sub(boundary.h as usize).unwrap_or(0);

    for (row, line) in chat.iter().skip(m).enumerate() {
        stdout.queue(cursor::MoveTo(0, row as u16)).unwrap();
        stdout.write(line.as_bytes());
    }
}

fn main() {
    let _ = crossterm::terminal::enable_raw_mode;
    let mut stdout = stdout();
    let (mut w, mut h) = terminal::size().unwrap();
    let mut bar = "═".repeat(w as usize);
    let label = "HEllo World";
    let mut prompt = String::new();
    let mut quit = false;
    let mut chat = Vec::new();

    while !quit {
        let r = crossterm::event::poll(Duration::ZERO).unwrap();
        while r {
            match read().unwrap() {
                crossterm::event::Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = "═".repeat(w as usize);
                }
                crossterm::event::Event::Key(event) => match event.code {
                    crossterm::event::KeyCode::Char(x) => {
                        if x == 'c'
                            && event
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL)
                        {
                            quit = true;
                            break;
                        } else {
                            prompt.push(x);
                        }
                    }
                    crossterm::event::KeyCode::Enter => {
                        chat.push(prompt.clone());
                        prompt.clear();
                    }
                    _ => {}
                },
                _ => (),
            }

            stdout
                .queue(terminal::Clear(ClearType::FromCursorUp))
                .unwrap();
            stdout.queue(terminal::Clear(ClearType::All)).unwrap();

            chat_window(
                &mut stdout,
                &chat,
                Rect {
                    x: 0,
                    y: 0,
                    w,
                    h: h, //h/2
                },
            );

            // stdout
            //  .queue(cursor::MoveTo(w / 2 - label.len() as u16 / 2, h / 2))
            //    .unwrap();
            // stdout.write(label.as_bytes()).unwrap();
            stdout.queue(cursor::MoveTo(0, h - 2)).unwrap();
            stdout.write(bar.as_bytes()).unwrap();
            stdout.queue(cursor::MoveTo(0, h)).unwrap();
            stdout.write(prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();
        }

        stdout.queue(cursor::MoveTo(0, h)).unwrap();
        stdout
            .queue(terminal::Clear(ClearType::FromCursorUp))
            .unwrap();
        stdout.write(b"out of the application....").unwrap();
        stdout.flush().unwrap();
    }
}
