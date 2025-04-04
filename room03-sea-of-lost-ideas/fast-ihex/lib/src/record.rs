//! A valid record is <anything>:<byte count (1u8)><address (2u8)><type (1u8)><data (max 255u8)>

#[derive(Debug)]
pub enum Record<'i> {
    Data {
        offset: u16,
        value: &'i [u8],
    },
    EndOfFile,
    #[cfg(feature = "i16hex")]
    ExtendedSegmentAddress(u16),
    #[cfg(feature = "i16hex")]
    StartSegmentAddress(()),
    #[cfg(feature = "i32hex")]
    ExtendedLinearAddress(u16),
    #[cfg(feature = "i32hex")]
    StartLinearAddress(u32),
    /// https://discuss.wayneandlayne.com/t/blinky-grid-serial-optical-bit-stream/161/2
    #[cfg(feature = "wayne-and-layne")]
    BlinkyTransmission(()),
    /// https://tech.microbit.org/software/spec-universal-hex/
    #[cfg(feature = "microbit")]
    BlockStart(()),
    #[cfg(feature = "microbit")]
    BlockEnd(()),
    #[cfg(feature = "microbit")]
    PaddedData(()),
    #[cfg(feature = "microbit")]
    CustomData(()),
    #[cfg(feature = "microbit")]
    OtherData(()),
}

#[derive(Debug)]
pub enum RecordType {
    Data = 0x00,
    EndOfFile = 0x01,
    #[cfg(feature = "i16hex")]
    ExtendedSegmentAddress = 0x02,
    #[cfg(feature = "i16hex")]
    StartSegmentAddress = 0x03,
    #[cfg(feature = "i32hex")]
    ExtendedLinearAddress = 0x04,
    #[cfg(feature = "i32hex")]
    StartLinearAddress = 0x05,
    /// https://discuss.wayneandlayne.com/t/blinky-grid-serial-optical-bit-stream/161/2
    #[cfg(feature = "wayne-and-layne")]
    BlinkyTransmission = 0x06,
    /// https://tech.microbit.org/software/spec-universal-hex/
    #[cfg(feature = "microbit")]
    BlockStart = 0x0A,
    #[cfg(feature = "microbit")]
    BlockEnd = 0x0B,
    #[cfg(feature = "microbit")]
    PaddedData = 0x0C,
    #[cfg(feature = "microbit")]
    CustomData = 0x0D,
    #[cfg(feature = "microbit")]
    OtherData = 0x0E,
}
