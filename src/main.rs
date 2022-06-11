mod particle;
mod system;

fn main() {
    let mut p = particle::Particle::new();
    p.pos[0] += 1.0;
    println!("Hello, world!");
    println!("{:?}", p);
    let s = system::System::new([10.0, 10.0, 10.0]);
    println!("{:?}", s);
}
