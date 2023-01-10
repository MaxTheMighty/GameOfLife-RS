
use std::{time::{Duration, Instant}};

fn main (){
    let mut five_seconds: Duration = Duration::from_secs(5);
    let mut now = Instant::now();
    while true {
        if(now.elapsed().as_secs() > 5){
            println!("Has been 5 seconds");
            break;
        }
    }
}