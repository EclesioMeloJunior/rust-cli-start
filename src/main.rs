use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyEvent, KeyCode},
    execute,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        Clear, 
        ClearType, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    },
    Result,
};
use std::io::{stdout, Write};
use std::time::Duration;

mod game;

fn main() -> Result<()>{
    enable_raw_mode();

    let mut stdout = stdout();

    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(0,1), (2, 2), (2, 3)]);

    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide,
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent {code , ..}) = read()? {
                match code {
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        } else {
            queue!(
                stdout,
                Clear(ClearType::All),
            )?;
            let mut i = 0;
            while let Some(line) = game.row_as_string(i) {
                queue!(stdout, MoveTo(0, i as u16), Print(line))?;
                i += 1;
            }

            queue!(
                stdout,
                MoveTo(0, (i + 1) as u16),
                Print("Press enter to exit...")
            )?;

            stdout.flush()?;
            game.tick();
        }
    }

    execute!(
        stdout,
        ResetColor,
        Show, 
        LeaveAlternateScreen,
    )?;

    disable_raw_mode()?;
    Ok(())
}
