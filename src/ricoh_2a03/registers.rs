//! The registers for the 6502/2A03.
//! This chip does not have generic registers
//! like R0 -> R10.

/// The register enums and their contained values.
/// More info at:
/// https://www.nesdev.org/wiki/CPU_registers
pub enum Register {
    /// Accumulator. Byte.
    A(u8),
    /// X index. Byte.
    X(u8),
    /// Y index. Byte.
    Y(u8),
    /// Program Counter. 2 Bytes.
    PC(u16),
    /// Stack Pointer. Byte.
    SP(u8),
    /// Status register. Only 6 bits used by ALU,
    /// but is byte wide. See:
    /// https://www.nesdev.org/wiki/Status_flags
    // TODO - maybe extrapolate Status Reg to own struct?
    SR(u8),
}