mod percentage_clock;

fn main() {
    
    loop {
        let dream = percentage_clock::get_time();
        println!("{dream}");
    }
    
}

