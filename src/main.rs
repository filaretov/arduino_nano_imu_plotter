mod constants;
mod data;
mod imu;

use std::io;
use std::io::Read;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

use crate::data::DataQueue;
use crate::imu::ImuReader;

const X_LIM: f64 = 256.0;

fn to_dataset(data: &Vec<(f64, f64)>, color: Color) -> Dataset {
    Dataset::default()
        .data(data)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .marker(Marker::Braille)
}

fn to_datasets(datasets: &[Vec<(f64, f64)>; 3]) -> Vec<Dataset> {
    vec![
        to_dataset(&datasets[0], Color::Red),
        to_dataset(&datasets[1], Color::Green),
        to_dataset(&datasets[2], Color::Blue),
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
        for _ in 0..2 {
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
                .block(Block::default().borders(Borders::ALL).title("Acc [g]"))
                .x_axis(Axis::default().bounds([0.0, X_LIM]))
                .y_axis(Axis::default().bounds([-5.0, 5.0]).labels(vec![
                    Span::raw("-5"),
                    Span::raw("0"),
                    Span::raw("5"),
                ]));
            f.render_widget(chart_acc, chunks[0]);
            let chart_gyr = Chart::new(gyr)
                .block(Block::default().borders(Borders::ALL).title("Gyr [deg/s]"))
                .x_axis(Axis::default().bounds([0.0, X_LIM]))
                .y_axis(Axis::default().bounds([-720.0, 720.0]).labels(vec![
                    Span::raw("-720"),
                    Span::raw("0"),
                    Span::raw("720"),
                ]));
            f.render_widget(chart_gyr, chunks[1]);
            let chart_mag = Chart::new(mag)
                .block(Block::default().borders(Borders::ALL).title("Mag [G]"))
                .x_axis(Axis::default().bounds([0.0, X_LIM]))
                .y_axis(Axis::default().bounds([-500.0, 500.0]).labels(vec![
                    Span::raw("-500"),
                    Span::raw("0"),
                    Span::raw("500"),
                ]));
            f.render_widget(chart_mag, chunks[2]);
        })?;

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                _ => (),
            }
        }
    }
}
