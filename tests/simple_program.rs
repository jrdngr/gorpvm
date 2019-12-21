use gorpvm::cpu::Cpu;

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
    cpu.run();

    assert_eq!(cpu.registers()[0], 1);
    assert_eq!(cpu.registers()[1], 8);
    assert_eq!(cpu.registers()[2], 9);
    assert_eq!(cpu.registers()[3], 0);
}
