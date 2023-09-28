use crossterm::{style::{Color, SetForegroundColor, Print, ResetColor}, execute};
use std::io::stdout;
use tabled::{builder::Builder, settings::Style};

#[derive(Clone)]
pub enum State {
    Ready,
    Waiting,
    Running,
    Terminated,
}

#[derive(Clone)]
pub struct Processes {
    pub id: usize,
    pub time_in: usize,
    pub state: State,
    pub job_units: usize,
    pub time_out: usize,
    pub priority: usize,
}

pub fn print_incolor(text: String, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    ).unwrap();
}

pub fn clear(){
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    #[cfg(target_os = "windows")]
    {
    print!("{esc}[2H", esc = 27 as char);
    print!("{esc}[2J", esc = 27 as char);
    }
}

pub fn print_proccesses_data(proceses_buf: &Vec<Processes>) {
    let mut builder = Builder::default();
    let headers = ["#", "Estado", "Trabajo Restante", "Tiempo de entrada", "Prioridad", "Tiempo de salida"];
    builder.set_header(headers);
    let mut counter = 1;

    for proccess in proceses_buf {
        let state:String = match proccess.state {
            State::Ready   => { "Ready".to_string() }
            State::Running => { "Running".to_string() }
            State::Waiting => { "Waiting".to_string() }
            State::Terminated => { "Terminated".to_string() }
        };

        let row = [ counter.to_string(), state, proccess.job_units.to_string() ,proccess.time_in.to_string(), proccess.priority.to_string(), proccess.time_out.to_string()];
        builder.push_record(row);
        counter += 1;
    }

    let table = builder.build()
    .with(Style::rounded()).to_string();
    println!("{table}");
}

pub fn print_fifo_data(fifo_queue: &Vec<usize>) {
    let mut builder = Builder::default();
    builder.set_header(["Cola Fifo"]);

    for index in fifo_queue {
        let row = [(index + 1).to_string()];
        builder.push_record(row);
    }
    let table = builder.build()
    .with(Style::rounded()).to_string();

    print_incolor("Fifo Queue: \n".to_string(), Color::DarkGreen);
    println!("{table}");
}
