use gorpvm::cpu::Cpu;

#[test]
fn looping_addition_program() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[
        1, 21, 0,
        1, 22, 1,
        5,
        2, 0, 22,
        1, 23, 1,
        11,
        3,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        1,
        4,
    ]);
    cpu.run();

    dbg!(&cpu.memory[0..30]);

    assert_eq!(cpu.memory[22], 4);
}
