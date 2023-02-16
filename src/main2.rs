use rand::Rng;
use std::io;
use std::io::Write;
use std::time::Instant;
fn _maina() {
    const LEN: usize = 1145;
    let mut a = [0; LEN];
    for i in 0..LEN {
        a[i] = rand::thread_rng().gen_range(1..=100);
    }
    let mut w = LEN - 1;
    let start = Instant::now();
    while w > 0 {
        let mut j = 0;
        while j < w {
            if a[j] < a[j + 1] {
                let v = a[j];
                a[j] = a[j + 1];
                a[j + 1] = v;
            }
            j += 1;
        }
        w -= 1;
    }
    let duration = start.elapsed();
    for i in a {
        print!("{} ", i);
        if io::stdout().flush().is_err() {
            println!("flush err")
        }
    }
    println! {"{:?}", duration};
}
