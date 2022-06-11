mod particle;

fn main() {
    let mut p = particle::Particle::new();
    p.pos[0] += 1.0;
    println!("Hello, world!");
    println!("{:?}", p);
}
