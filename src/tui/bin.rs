// use colored::Colorize;
// use crossterm::{
//     cursor,
//     event::{self, Event, KeyCode, KeyEvent},
//     execute, queue, style,
//     style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//     terminal::EnterAlternateScreen,
//     terminal::{self, ClearType, LeaveAlternateScreen},
//     Command, QueueableCommand, Result,
// };
use core::{num, slice};
use std::{
    collections::HashMap,
    fmt::{format, Display},
    io::{self, Write},
    ops::{Index, IndexMut},
    rc::Rc,
    thread::{self, current},
    time::Duration,
};

use math_library::{
    self,
    matrix::{Direction, PosElem},
    Matrix,
};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders, Widget},
//     Frame, Terminal,
// };

macro_rules! slice_array {
    ($($e:tt),*) => {
        [$(slice_array!($e; f64)),*].as_slice()
    };
    ([$($e:expr),*]; $t:ty) => {
        [$($e as $t),*].as_slice()
    }
}

fn solve_lp() {
    let source_task = Matrix::from(slice_array!(
        [0, -14, -18, 0, 0, 0],
        [-168, 10, 8, 1, 0, 0],
        [180, 5, 10, 0, 1, 0],
        [144, 6, 12, 0, 0, 1]
    ));
    let mut resolve_task = source_task.clone();
    resolve_task.simplex(0);
    println!("Resolve task: {}", resolve_task);
}

fn main() {
    // solve_lp();
    let mat = Matrix::new_simplex_table(
        slice_array!([2,3,4];f64),
        &[('=', 2f64), ('≤', 3f64)],
        slice_array!([2, 2, 2], [2, 2, 2]),
    );
    println!("{}", mat);

    // let source_task = Matrix::from(slice_array!(
    //     [0, 6.5, 0, -7.5, 23.5, -5],
    //     [12, 1, 3, 1, 4, -1],
    //     [14, 2, 0, -1, 12, -1],
    //     [6, 1, 2, 0, 3, -1]
    // ));
    // let mut resolve_task = source_task.clone();
    // resolve_task.simplex(0);
    // println!("Resolve task: {}", resolve_task);
}

// fn main() -> Result<()> {
//     // setup terminal
//     let mut stdout = io::stdout();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     input_matrix(terminal.backend_mut())?;
//     // run(terminal.backend_mut());

//     Ok(())
// }

// fn run<W>(w: &mut W) -> Result<()>
// where
//     W: Write,
// {
//     let mut health = 100;
//     execute!(w, EnterAlternateScreen);
//     terminal::enable_raw_mode();
//     loop {
//         thread::sleep(Duration::from_millis(1000));
//         if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
//             if code == KeyCode::Char('q') {
//                 break;
//             }
//             queue!(
//                 w,
//                 SetForegroundColor(Color::DarkBlue),
//                 SetBackgroundColor(Color::DarkCyan),
//                 Print(format!("{:?}", code))
//             )?;
//         }
//         w.flush()?;
//         // match read_char()? {
//         //     'd' => health -= 10,
//         //     'q' => break,
//         //     _ => {}
//         // }
//     }
//     execute!(
//         w,
//         style::ResetColor,
//         cursor::Show,
//         terminal::LeaveAlternateScreen
//     )?;
//     terminal::disable_raw_mode()?;
//     Ok(())
// }
// pub fn input_matrix<W>(w: &mut W) -> Result<Matrix>
// where
//     W: Write,
// {
//     terminal::enable_raw_mode()?;
//     let mut input_row_col = true;
//     let mut row_str = "".to_owned();
//     let mut col_str = "".to_owned();
//     execute!(w, EnterAlternateScreen);
//     // let mut row
//     loop {
//         queue!(
//             w,
//             ResetColor,
//             terminal::Clear(ClearType::All),
//             cursor::MoveTo(1, 1),
//             cursor::Hide,
//             Print("Enter size matrix: "),
//             SetForegroundColor(if input_row_col {
//                 Color::Red
//             } else {
//                 Color::White
//             }),
//             Print("row - "),
//             ResetColor,
//             Print(format!("{}; ", row_str)),
//             SetForegroundColor(if input_row_col == false {
//                 Color::Red
//             } else {
//                 Color::White
//             }),
//             Print("col - "),
//             ResetColor,
//             Print(format!("{};", col_str)),
//             Print("\t Press TAB to toggle input".italic())
//         )?;
//         w.flush()?;
//         if let Event::Key(KeyEvent { code, .. }) = event::read()? {
//             match code {
//                 KeyCode::Enter => break,
//                 KeyCode::Tab => input_row_col = !input_row_col,

//                 KeyCode::Char(c) => {
//                     if c.is_numeric() {
//                         match input_row_col {
//                             true => row_str.push(c),
//                             false => col_str.push(c),
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
//     execute!(w, LeaveAlternateScreen, cursor::Show)?;
//     terminal::disable_raw_mode()?;
//     todo!()
// }
// pub fn key_pressed(key: KeyCode) -> bool {
//     if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
//         return code == key;
//     }
//     false
// }

// pub fn read_char() -> Result<char> {
//     loop {
//         if let Event::Key(KeyEvent {
//             code: KeyCode::Char(c),
//             ..
//         }) = event::read()?
//         {
//             return Ok(c);
//         }
//     }
// }
// pub fn read_line() -> Result<String> {
//     let mut line = String::new();
//     while let Event::Key(KeyEvent { code, .. }) = event::read()? {
//         match code {
//             KeyCode::Enter => {
//                 break;
//             }
//             KeyCode::Char(c) => {
//                 line.push(c);
//             }
//             _ => {}
//         }
//     }

//     Ok(line)
// }
