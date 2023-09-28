use crate::utils::*;
use std::{thread, time};
use crossterm::style::Color;

pub fn round_robin_algorithm(process_pool: &mut Vec<Processes>) {
    let q = 3;

    let mut q_counter = 0;
    let mut time = 0;

    let process_pool_initial = process_pool.clone();
    let mut pre_fifo_queue: Vec<usize> = Vec::new();
    let mut fifo_queue: Vec<usize> = Vec::new();

    loop {
        for (index, process) in process_pool.iter().enumerate() {
            if process.time_in == time {
                pre_fifo_queue.push(index);
            }
        }

        if pre_fifo_queue.len() != 0 {
            pre_fifo_queue.sort_by(|a, b| process_pool[*a].priority.cmp(&process_pool[*b].priority));
        }

        for index in &pre_fifo_queue {
            fifo_queue.push(*index);
        }

        pre_fifo_queue.clear();

        let mut running_procces: &mut Processes;

        if fifo_queue.len() != 0 {

            for index in &fifo_queue {
                process_pool[*index].state = State::Waiting;
            }
            
            running_procces = &mut process_pool[fifo_queue[0]];

            if running_procces.job_units == 0 {
                running_procces.state = State::Terminated;
                running_procces.time_out = time;
                fifo_queue.remove(0);
                q_counter  = q;
                
                if fifo_queue.len() != 0 {
                    running_procces = &mut process_pool[fifo_queue[0]];
                }
            }
            
            if q_counter == 0 {
                let swap = fifo_queue[0];
                fifo_queue.remove(0);
                fifo_queue.push(swap);
                running_procces.state = State::Waiting;
                q_counter = q; 
                if fifo_queue.len() != 0 {
                    running_procces = &mut process_pool[fifo_queue[0]];
                }
            }

            match running_procces.state {
                State::Terminated => {}
                _ => {
                    running_procces.state = State::Running;
                    running_procces.job_units -= 1;
                }
            }
            
        }

        
        let sec = time::Duration::from_secs(1);
        thread::sleep(sec);

        print_incolor("Tiempo: ".to_owned(), Color::DarkGreen);
        println!("{time}\n");
        print_incolor("q: ".to_owned(), Color::DarkGreen);
        println!("{q_counter}\n");
        print_incolor("Processes Initial State\n".to_string(), Color::DarkGreen);
        print_proccesses_data(&process_pool_initial);
        println!("");
        print_incolor("Processes State\n".to_string(), Color::DarkGreen);
        print_proccesses_data(&process_pool);
        println!("");
        print_fifo_data(&fifo_queue);

        if fifo_queue.len() != 0 {
            q_counter -= 1;
        }

        time += 1;
        
        let mut checker = 0;

        for process in process_pool.clone() {
            match process.state {
                State::Terminated => {
                    checker += 1
                },
                _  => {
                    clear();
                }
            }

            if checker == process_pool.len() {
                return;
            }
        }


    }
}