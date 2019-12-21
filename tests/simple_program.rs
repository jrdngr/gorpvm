use gorpvm::cpu::Cpu;

#[test]
fn looping_addition_program() {
    let mut cpu = Cpu::new();
    let program = "
    set 1 0
    set 1 1
    add 0r 1r 1
    let 1r 8 3
    jpt 3r 0 3
    jpf 2r 1 2
    hlt
    hlt
    set 9 2
    ";
    cpu.load_assembly(program);
    cpu.run();

    assert_eq!(cpu.registers()[0], 1);
    assert_eq!(cpu.registers()[1], 8);
    assert_eq!(cpu.registers()[2], 9);
    assert_eq!(cpu.registers()[3], 0);
}
