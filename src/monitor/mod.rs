pub mod streams;
pub mod types;
pub mod operation_eval;
// pub mod instrumentation;

#[cfg(test)]
mod streams_test;
#[cfg(test)]
mod operation_eval_test;

use std::error::Error;
use crate::{errors, monitor::streams::{IoTStream, OutputStream}, program::Program};
use tokio::time::{Duration, interval};
use std::time::Instant;


use colored::Colorize;

/*
 *  TODO:
 *  1. Make algorithm for calculating size of IoTStream
 *  2. Make algorithm for operation_eval
 *  3. Fix tests
 *  4. Integrate instrumentation
 *  5. 
  * */

type MonitorElement = Result<(usize, bool), Box<dyn Error>>;

impl Program {
    pub async fn monitor(&mut self, time_interval: i128, speed: bool) -> Result<(), Box<dyn Error>> {
        
        let Some(streams) = &mut self.environment else { return Err(errors::Error::FieldNotPresent.into()); };
        let Some(size) = self.iotstream_len else { return Err(errors::Error::FieldNotPresent.into()); };

        let mut interval = interval(Duration::from_millis(time_interval as u64));

        let mut t = 0;

        let mut cur_idx = 0;
        let mut devices = IoTStream::with_capacity(size);

        #[cfg(not(debug_assertions))]
        {
            println!("-----------------------------------");
            println!("Started monitoring Home Assistant");
            println!("Violations will be printed below:");
            println!("-----------------------------------");
        }
        
        loop {
            if !speed{
                interval.tick().await;
            }
            
            let start = Instant::now();
            #[cfg(debug_assertions)]
            println!("--- Interval {:<4}", format!("{}",t).blue().bold());

            //Todo: This should be fixed
            if cfg!(debug_assertions) {     
                // devices.push_at(instrumentation.fetch_device_states().await, cur_idx);
                devices.push_at(todo!(), cur_idx)?;
            } else {
                devices.push_at(todo!(), cur_idx)?;
            }
            cur_idx = (cur_idx + 1) % size;
            
            

            async {
                for el in Self::monitor_logic(streams, &t, &devices) {
                    let (prop_num, _ )=  el?; 
                    let msg = format!("Prop {} violated", prop_num + 1);
                    println!("\t{} at time: {}", msg.red().bold().underline(), format!("{}s",t).red().bold());
                }
                t += time_interval / 1000;
                
                Ok::<(), Box<dyn Error>>(())
            }.await?;
            #[cfg(debug_assertions)]
            {
                let elapsed = start.elapsed();
                let colored_time = if elapsed.as_millis() > time_interval as u128 { format!("{:?}",elapsed).red().bold() } 
                    else { format!("{:?}",elapsed).bright_green().bold() };
                println!("\tExecution Time: {}", colored_time);
            }

        }
    }

    pub fn monitor_logic<'a>(env: &'a mut [OutputStream], t: &'a i128, device_stream: &'a IoTStream) -> Box<dyn Iterator<Item = MonitorElement> + 'a> {
        Box::new(
            env
                .iter_mut()
                .enumerate()
                .map(|(prop_num, output_stream)| {
                    let t = *t;
                    
                    // SDI update
                    output_stream.insert(t); 

                    // Calculate the new state of the streams
                    output_stream.update(t, device_stream)?; 

                    // Give verdicts
                    let is_violated = output_stream.get_violated_verdict_single();
                    
                    output_stream.clean_up();

                    Ok((prop_num, is_violated))
                }).filter(|el| el.as_ref().map(|(_, v)| *v).unwrap_or(true))
        )
    }
}
