//! The addressing modes and other shared 
//! utilities for such modes.

/// All the possible addressing modes for the 
/// 6502/2A03. Described in:
/// https://www.nesdev.org/wiki/CPU_addressing_modes
// TODO: Add doc-comments for each addr mode
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Implicit,
    Accumulator,
}

