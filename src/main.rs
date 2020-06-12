mod bus;
mod cpu;

fn main() {
    let b = bus::Bus::new();
    println!("{}", b.ram.len());
}
