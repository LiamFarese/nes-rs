use crate::cpu::CPU;

mod arithmetic;
mod helpers;
mod increments;
mod load_store;
mod logical;
mod transfers;

fn with_cpu<F>(program: &[u8], setup: F) -> CPU
where
    F: FnOnce(&mut CPU),
{
    let mut cpu = CPU::new();
    cpu.load(program);
    cpu.reset();
    setup(&mut cpu);
    cpu.run();
    cpu
}
