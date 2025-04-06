type Block = Box<[u8]>;

static VERBOSE: bool = false;

fn run() {
    let mut blocks = Vec::<Block>::new();
    loop {
        if VERBOSE {
            print!("[");
            for b in blocks.iter() {
                print!("{}, ", b.len());
            }
            println!("]");
        }

        if fastrand::f32() < 0.25 && blocks.len() > 4 {
            blocks.drain((blocks.len()-2)..).for_each(drop);
        }

        let mut block = vec![0; fastrand::usize(500..3000)];
        fastrand::fill(&mut block);
        blocks.push(block.into_boxed_slice());

        let d = std::time::Duration::from_millis(fastrand::u64(0..1500));
        std::thread::sleep(d);
    }
}
fn main() {
    loop {
        if fastrand::f32() < 0.1 {
            std::thread::spawn(run);
        }
        std::thread::sleep(std::time::Duration::from_secs(4));
    }
}
