mod particle;
mod system;
mod integrator;

fn main() {
    let mut p = particle::Particle::new();
    p.pos[0] += 1.0;
    println!("Hello, world!");
    println!("{:?}", p);
    let mut s = system::System::new([10.0, 10.0, 10.0]);
    s.particles.push(p);
    println!("{:?}", s);
    let i = integrator::Integrator::new(0.1);
    println!("{:?}", i);
    i.run(&mut s, 91);
    println!("{:?}", s);
}
