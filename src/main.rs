use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

struct OnDrop;
impl Drop for OnDrop {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        stdout().execute(crossterm::terminal::LeaveAlternateScreen);
        stdout().execute(crossterm::cursor::Show);
        crossterm::terminal::disable_raw_mode();
    }
}

fn try_center_message(
    s: &str,
    stdout: &mut std::io::Stdout,
    columns: u16,
    rows: u16,
) -> std::io::Result<bool> {
    let middle_row = rows / 2;
    let middle_column = columns / 2;
    if s.len() + 3 < columns.into() {
        let start_columns = middle_column - (s.len() as u16 / 2);
        stdout.queue(crossterm::cursor::MoveTo(start_columns, middle_row))?;
        write!(stdout, "{s}")?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn render(stdout: &mut std::io::Stdout, columns: u16, rows: u16) -> std::io::Result<()> {
    stdout.queue(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;
    stdout.queue(crossterm::cursor::MoveTo(0, 0))?;
    for i in 1..=columns {
        let hundreds = i / 100;
        let i = i % 100;
        let tens = i / 10;
        let ones = i % 10;
        if hundreds == 0 {
            if tens == 0 {
                write!(stdout, "{ones}")?;
            } else {
                stdout.queue(crossterm::cursor::SavePosition)?;
                write!(stdout, "{tens}")?;
                stdout.queue(crossterm::cursor::RestorePosition)?;
                stdout.queue(crossterm::cursor::MoveDown(1))?;
                write!(stdout, "{ones}")?;
                stdout.queue(crossterm::cursor::RestorePosition)?;
                stdout.queue(crossterm::cursor::MoveRight(1))?;
            }
        } else {
            stdout.queue(crossterm::cursor::SavePosition)?;
            write!(stdout, "{hundreds}")?;
            stdout.queue(crossterm::cursor::RestorePosition)?;
            stdout.queue(crossterm::cursor::MoveDown(1))?;
            write!(stdout, "{tens}")?;
            stdout.queue(crossterm::cursor::RestorePosition)?;
            stdout.queue(crossterm::cursor::MoveDown(1))?;
            stdout.queue(crossterm::cursor::MoveDown(1))?;
            write!(stdout, "{ones}")?;
            stdout.queue(crossterm::cursor::RestorePosition)?;
            stdout.queue(crossterm::cursor::MoveRight(1))?;
        }
    }
    for i in 1..=rows {
        stdout.queue(crossterm::cursor::MoveTo(0, i - 1))?;
        write!(stdout, "{i}")?;
    }
    #[allow(clippy::collapsible_if)]
    if rows > 3 {
        if !try_center_message(
            &format!("{columns} columns x {rows} rows"),
            stdout,
            columns,
            rows,
        )? {
            if !try_center_message(&format!("{columns} x {rows}"), stdout, columns, rows)? {
                try_center_message(&format!("{columns}x{rows}"), stdout, columns, rows)?;
            }
        }
    }
    stdout.flush()
}

fn main() -> std::io::Result<()> {
    let _on_drop = OnDrop;
    let mut stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    stdout.queue(crossterm::terminal::EnterAlternateScreen)?;
    stdout.queue(crossterm::cursor::Hide)?;
    let (columns, rows) = crossterm::terminal::size()?;
    render(&mut stdout, columns, rows)?;
    loop {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) => match key_event.code {
                crossterm::event::KeyCode::Backspace
                | crossterm::event::KeyCode::Enter
                | crossterm::event::KeyCode::Home
                | crossterm::event::KeyCode::End
                | crossterm::event::KeyCode::Delete
                | crossterm::event::KeyCode::Esc
                | crossterm::event::KeyCode::Char('q')
                | crossterm::event::KeyCode::Char('Q') => {
                    break;
                }
                _ => {}
            },
            crossterm::event::Event::Resize(new_columns, new_rows) => {
                render(&mut stdout, new_columns, new_rows)?;
            }
            _ => {}
        }
    }

    Ok(())
}
