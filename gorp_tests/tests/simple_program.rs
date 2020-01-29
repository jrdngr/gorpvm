use gorp_cpu::Cpu;

#[test]
fn looping_addition_program() {
    let mut cpu = Cpu::new();
    let program = "
    set 0 1
    set 1 1
    add 1 0r 1r
    let 3 1r 8
    jpt 3 3r 0
    jpf 2 2r 1 
    hlt
    hlt
    set 2 9
    ";
    cpu.load_assembly(program);
    looping_addition_test(cpu);
}

#[test]
fn looping_addition_from_file() {
    let mut cpu = Cpu::new();
    cpu.load_assembly_file("./tests/resources/simple_program.gas");
    looping_addition_test(cpu);
}

fn looping_addition_test(mut cpu: Cpu) {
    cpu.run();

    assert_eq!(cpu.registers()[0], 1);
    assert_eq!(cpu.registers()[1], 8);
    assert_eq!(cpu.registers()[2], 9);
    assert_eq!(cpu.registers()[3], 0);
}
