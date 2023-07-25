pub enum Instruction {
    // 8-bit arithmetic and logical instructions
    ADD8(ArithmeticTarget)
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}