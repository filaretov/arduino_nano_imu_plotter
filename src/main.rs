mod constants;
mod data;
mod imu;

use std::io;
use std::io::Read;
use termion::raw::IntoRawMode;
use termion::{async_stdin, event::Key, input::TermRead};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use tui::Terminal;

use crate::data::DataQueue;
use crate::imu::ImuReader;

fn to_dataset(data: &Vec<(f64, f64)>, color: Color) -> Dataset {
    Dataset::default()
        .data(data)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .marker(Marker::Braille)
}

fn to_datasets<'a>(datasets: &'a [Vec<(f64, f64)>; 3]) -> Vec<Dataset> {
    vec![
        to_dataset(&datasets[0], Color::Red),
        to_dataset(&datasets[1], Color::Blue),
        to_dataset(&datasets[2], Color::Green),
    ]
}

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    let mut reader = ImuReader::new();
    let mut acc_queue = DataQueue::new();
    let mut gyr_queue = DataQueue::new();
    let mut mag_queue = DataQueue::new();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut asi = async_stdin();
    terminal.clear().expect("Couldn't clear the screen");
    loop {
        for _ in 0..4 {
            let imu = reader.read();
            acc_queue.push(imu.acc[0] as f64, imu.acc[1] as f64, imu.acc[2] as f64);
            gyr_queue.push(imu.gyr[0] as f64, imu.gyr[1] as f64, imu.gyr[2] as f64);
            mag_queue.push(imu.mag[0] as f64, imu.mag[1] as f64, imu.mag[2] as f64);
        }
        let acc = acc_queue.get();
        let gyr = gyr_queue.get();
        let mag = mag_queue.get();
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(size);
            let acc = to_datasets(&acc).to_vec();
            let gyr = to_datasets(&gyr).to_vec();
            let mag = to_datasets(&mag).to_vec();
            let chart_acc = Chart::new(acc)
                .block(Block::default().borders(Borders::ALL).title("Acc"))
                .x_axis(Axis::default().bounds([0.0, 128.0]))
                .y_axis(Axis::default().bounds([-5.0, 5.0]));
            f.render_widget(chart_acc, chunks[0]);
            let chart_gyr = Chart::new(gyr)
                .block(Block::default().borders(Borders::ALL).title("Gyr"))
                .x_axis(Axis::default().bounds([0.0, 128.0]))
                .y_axis(Axis::default().bounds([-720.0, 720.0]));
            f.render_widget(chart_gyr, chunks[1]);
            let chart_mag = Chart::new(mag)
                .block(Block::default().borders(Borders::ALL).title("Mag"))
                .x_axis(Axis::default().bounds([0.0, 128.0]))
                .y_axis(Axis::default().bounds([-200.0, 200.0]));
            f.render_widget(chart_mag, chunks[2]);
        })?;

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                // If any of them is q, quit
                Key::Char('q') => {
                    // Clear the terminal before exit so as not to leave
                    // a mess.
                    terminal.clear()?;
                    return Ok(());
                }
                // Otherwise, throw them away.
                _ => (),
            }
        }
    }
}
