/* === Memory === */
/// Memory is addressed with 32 bits addresses
pub const MEMORY_MAX_ADDRESSABLE_ADDRESS: usize = u32::MAX as usize;

/* === Registers === */

/// Application Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Apsr(u32);

/// Interrupt Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Ipsr(u32);

/// Execution Program Status Register
/// <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
pub struct Epsr(u32);

/// List of the registers:
/// - General purpose registers R0-R12.
/// - Two Stack Pointer registers, `SP_main` and `SP_process`. These are banked versions of SP, also described as R13.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-ARM-core-registers>
/// - The Link Register, `LR` also described as R14.
/// - The Program Counter, `PC`, sometimes described as R15.
/// - Status registers for flags, execution state bits, and the current exception number.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-program-status-registers--xPSR>
/// - A mask register, `PRIMASK`, used to manage the prioritization scheme for exceptions and interrupts.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-mask-register--PRIMASK>
/// - A control register, `CONTROL` that identifies the current stack.
///   <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-CONTROL-register>
///
/// Source: <https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers>
#[allow(nonstandard_style)]
pub enum Register {
    R0(u32),
    R1(u32),
    R2(u32),
    R3(u32),
    R4(u32),
    R5(u32),
    R6(u32),
    R7(u32),
    R8(u32),
    R9(u32),
    R10(u32),
    R11(u32),
    R12(u32),
    /// Stack Pointer register, banked version of SP (with SpProcess, described as R13), also described as MSP
    SP_main(u32),
    /// Stack Pointer register, banked version of SP (with SpMain, described as R13), also described as PSP
    SP_process(u32),
    /// The Link Register, also described as R14
    LR(u32),
    /// The Program Counter, also described as R15
    PC(u32),
    /// A mask register, used to manage the prioritization scheme for exceptions and interrupts
    PRIMASK(u32),
    /// A control register, that identifies the current stack
    CONTROL(u32),
    /// Application Program Status Register
    APSR(Apsr),
    /// Interrupt Program Status Register
    IPSR(Ipsr),
    /// Execution Program Status Register
    EPSR(Epsr),
}

/* === Instruction set === */
pub enum Instruction {
    Bits32(Instruction32),
    Bits16(Instruction16),
}

/// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions>
#[allow(nonstandard_style)]
pub enum Instruction16 {
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADC--register->
    ADC,
    ADD,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADR>
    ADR,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/AND--register->
    AND,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ASR--register->
    ASR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/B>
    B,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BIC--register->
    BIC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BKPT>
    BKPT,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BLX--register->
    BLX,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BX>
    BX,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMN--register->
    CMN,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CMP--register->
    CMP,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPS>
    CPS,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/CPY>
    CPY,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/EOR--register->
    EOR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDM,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDMIA,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDM--LDMIA--LDMFD>
    LDMFD,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--immediate->
    /// Literal: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--literal->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDR--register->
    LDR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRB--register->
    LDRB,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRH--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRH--register->
    LDRH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRSB--register->
    LDRSB,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LDRSH--register->
    LDRSH,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSL--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSL--register->
    LSL,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/LSR--register->
    LSR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--register->
    /// Shifted Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MOV--shifted-register->
    MOV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MUL>
    MUL,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MVN--register->
    MVN,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/NEG>
    NEG,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/NOP>
    NOP,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ORR--register->
    ORR,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/POP>
    POP,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/PUSH>
    PUSH,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REV>
    REV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REV16>
    REV16,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/REVSH>
    REVSH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ROR--register->
    ROR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/RSB--immediate->
    RSB,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SBC--register->
    SBC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SEV>
    SEV,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STM,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STMIA,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STM--STMIA--STMEA>
    STMEA,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STR--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STR--register->
    STR,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRB--register->
    STRB,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRH--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/STRH--register->
    STRH,
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--immediate->
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--register->
    /// SP minux immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SUB--SP-minus-immediate->
    SUB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SVC>
    SVC,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SXTB>
    SXTB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/SXTH>
    SXTH,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/TST--register->
    TST,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UDF>
    UDF,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UXTB>
    UXTB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/UXTH>
    UXTH,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/WFE>
    WFE,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/WFI>
    WFI,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/YIELD>
    YIELD,
}

#[allow(nonstandard_style)]
pub enum Instruction32 {
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/BL>
    BL,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DMB>
    DMB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/DSB>
    DSB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ISB>
    ISB,
    /// <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MRS>
    MRS,
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/MSR--register->
    MSR,
}
