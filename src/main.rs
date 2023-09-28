use std::io::stdin;
use crossterm::style::Color;
use tabled::{builder::Builder, settings::Style };
use rand::{self, Rng};

use scheduling_sim::utils::*;
use scheduling_sim::algorithms::{
    round_robin::*,
    fcfs::*,
    srtf::*,
    str::*
};

enum Algorithms {
    RoundRobin,
    FCFS,
    STR,
    SRTF,
}


fn ask_algorithm (input_buf: &mut String) -> Result<Algorithms, ()> {
    let mut builder = Builder::default();
    
    let rows = [ ["#", "Algorithm"], ["1", "First Come First Serve"], ["2", "Round Robin"], ["3", "STR -> todo"], ["4", "SRTF -> todo"] ]; 
    for row in rows { builder.push_record(row); };
    let table = builder.build()
    .with(Style::rounded()).to_string();

    println!("{table}");

    print_incolor("[?] ".to_string(), Color::Cyan);
    println!("Algoritmo a usar: ");
    print_incolor("> ".to_string(), Color::Green);
    stdin().read_line(input_buf).unwrap();

    match input_buf.trim() {
        "1" => { return Ok(Algorithms::FCFS); },
        "2" => { return Ok(Algorithms::RoundRobin) ;},
        "3" => { return Ok(Algorithms::SRTF);},
        "4" => { return Ok(Algorithms::STR);},
        _   => { return Err(());}
    }
}

fn init_proccesses(proccess_pool: &mut Vec<Processes>){
    let mut priority;
    let mut priority_list = Vec::new();
    for i in 0..6 { 
        loop {
            priority = rand::thread_rng().gen_range(1..10);

            if priority_list.contains(&priority) {
                continue;
            }else{
                priority_list.push(priority);
                break;
            }
        }

        let proccess = Processes {
            id: i + 1,
            time_in: rand::thread_rng().gen_range(1..12),
            state: State::Ready,
            time_out: 0,
            priority: priority,
            job_units: rand::thread_rng().gen_range(1..10),
        };
        proccess_pool.push(proccess);   
    }
}

fn main() {
    let mut process_pool: Vec<Processes> = Vec::new();
    init_proccesses(&mut process_pool);
    let mut input_buf: String = String::new();
    let algorithm_result = ask_algorithm(&mut input_buf);
    
    let algorithm: Algorithms = match algorithm_result {
        Err(_) => {
            print_incolor("[!] Error: ".to_string(), Color::Red);
            println!("Selecciona un algoritmo comprendido entre las 4 opciones mostradas. [ input esperado => (1 - 4) ]");
            return;
        },
        Ok(algorithm) => {
            algorithm
        },
    };

    clear();

    match algorithm {
        Algorithms::RoundRobin => { round_robin_algorithm(&mut process_pool) },
        Algorithms::FCFS => { fcfs_algorithm(&mut process_pool)},
        Algorithms::SRTF => { srtf_algorithm(&mut process_pool) },
        Algorithms::STR  => { str_algorithm(&mut process_pool) }
    }


}

