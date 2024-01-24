use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, QueueableCommand};
use std::{char, io};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

////////////////////////////////////////////////////////////////////////////

struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn chat_window(stdout: &mut io::Stdout, chat: &[String], boundary: Rect) {
    let n = chat.len();
    let m = n.checked_sub(boundary.h as usize).unwrap_or(0);

    for (row, line) in chat.iter().skip(m).enumerate() {
        stdout.queue(cursor::MoveTo(0, row as u16)).unwrap();
        let bytes = line.as_bytes();
        if bytes.len() < boundary.w as usize {
            stdout.write(bytes).unwrap();
        } else {
            stdout.write(&bytes[0..boundary.w]).unwrap(); //need correcttoon
        }
    }
}

fn main() {
    let mut stdout = stdout();
    let _ = crossterm::terminal::enable_raw_mode;
    let (mut w, mut h) = terminal::size().unwrap();
    let mut bar = "═".repeat(w as usize);
    let mut prompt = String::new();
    let mut quit = false;
    let mut chat = Vec::new();

    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = "═".repeat(w as usize);
                }
                Event::Key(event) => match event.code {
                    KeyCode::Char(x) => {
                        prompt.push(x);
                    }
                    KeyCode::Enter => {
                        chat.push(prompt.clone());
                        prompt.clear();
                    }
                    KeyCode::Esc => {
                        prompt.clear();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        stdout
            .queue(terminal::Clear(ClearType::FromCursorUp))
            .unwrap();
        

        chat_window(
            &mut stdout,
            &chat,
            Rect {
                x: 0,
                y: 0,
                w: w as usize,
                h: h as usize, //h/2
            },
        );

        stdout.queue(cursor::MoveTo(0, h - 2)).unwrap();
        stdout.write(bar.as_bytes()).unwrap();

        stdout.queue(cursor::MoveTo(0, h - 1)).unwrap();
        stdout.write(prompt.as_bytes()).unwrap();

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(33));
    }
}
