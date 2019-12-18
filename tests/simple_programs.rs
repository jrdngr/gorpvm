use gorpvm::cpu::Cpu;

#[test]
fn looping_addition_program() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[
        0x03, 0b1000_0001, 0, 0,
        0x03, 0b1000_0001, 0, 1,
        0x20, 0, 1, 1,
        0x34, 1, 0b1000_1000, 3,
        0x10, 3, 0b1000_0001, 0b1000_0100,
        0, 0, 0, 0,
    ]);
    cpu.run();

    dbg!(&cpu.registers());

    assert_eq!(cpu.registers()[2], 8);
}
