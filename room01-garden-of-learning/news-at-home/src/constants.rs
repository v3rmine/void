// Internal character sets used with ESC R n

/// American character set
pub const CHARSET_USA: u8 = 0;
/// French character set
pub const CHARSET_FRANCE: u8 = 1;
/// German character set
pub const CHARSET_GERMANY: u8 = 2;
/// UK character set
pub const CHARSET_UK: u8 = 3;
/// Danish character set 1
pub const CHARSET_DENMARK1: u8 = 4;
/// Swedish character set
pub const CHARSET_SWEDEN: u8 = 5;
/// Italian character set
pub const CHARSET_ITALY: u8 = 6;
/// Spanish character set 1
pub const CHARSET_SPAIN1: u8 = 7;
/// Japanese character set
pub const CHARSET_JAPAN: u8 = 8;
/// Norwegian character set
pub const CHARSET_NORWAY: u8 = 9;
/// Danish character set 2
pub const CHARSET_DENMARK2: u8 = 10;
/// Spanish character set 2
pub const CHARSET_SPAIN2: u8 = 11;
/// Latin American character set
pub const CHARSET_LATINAMERICA: u8 = 12;
/// Korean character set
pub const CHARSET_KOREA: u8 = 13;
/// Slovenian character set
pub const CHARSET_SLOVENIA: u8 = 14;
/// Croatian character set
pub const CHARSET_CROATIA: u8 = 14;
/// Chinese character set
pub const CHARSET_CHINA: u8 = 15;

// Character code tables used with ESC t n

/// USA, Standard Europe character code table
pub const CODEPAGE_CP437: u8 = 0;
/// Katakana (Japanese) character code table
pub const CODEPAGE_KATAKANA: u8 = 1;
/// Multilingual character code table
pub const CODEPAGE_CP850: u8 = 2;
/// Portuguese character code table
pub const CODEPAGE_CP860: u8 = 3;
/// Canadian-French character code table
pub const CODEPAGE_CP863: u8 = 4;
/// Nordic character code table
pub const CODEPAGE_CP865: u8 = 5;
/// Cyrillic character code table
pub const CODEPAGE_WCP1251: u8 = 6;
/// Cyrillic #2 character code table
pub const CODEPAGE_CP866: u8 = 7;
/// Cyrillic/Bulgarian character code table
pub const CODEPAGE_MIK: u8 = 8;
/// East Europe, Latvian 2 character code table
pub const CODEPAGE_CP755: u8 = 9;
/// Iran 1 character code table
pub const CODEPAGE_IRAN: u8 = 10;
/// Hebrew character code table
pub const CODEPAGE_CP862: u8 = 15;
/// Latin 1 character code table
pub const CODEPAGE_WCP1252: u8 = 16;
/// Greek character code table
pub const CODEPAGE_WCP1253: u8 = 17;
/// Latin 2 character code table
pub const CODEPAGE_CP852: u8 = 18;
/// Multilingual Latin 1 + Euro character code table
pub const CODEPAGE_CP858: u8 = 19;
/// Iran 2 character code table
pub const CODEPAGE_IRAN2: u8 = 20;
/// Latvian character code table
pub const CODEPAGE_LATVIAN: u8 = 21;
/// Arabic character code table
pub const CODEPAGE_CP864: u8 = 22;
/// West Europe character code table
pub const CODEPAGE_ISO_8859_1: u8 = 23;
/// Greek character code table
pub const CODEPAGE_CP737: u8 = 24;
/// Baltic character code table
pub const CODEPAGE_WCP1257: u8 = 25;
/// Thai character code table
pub const CODEPAGE_THAI: u8 = 26;
/// Arabic character code table
pub const CODEPAGE_CP720: u8 = 27;
/// Cyrillic character code table
pub const CODEPAGE_CP855: u8 = 28;
/// Turkish character code table
pub const CODEPAGE_CP857: u8 = 29;
/// Central Europe character code table
pub const CODEPAGE_WCP1250: u8 = 30;
/// Baltic character code table
pub const CODEPAGE_CP775: u8 = 31;
/// Turkish character code table
pub const CODEPAGE_WCP1254: u8 = 32;
/// Hebrew character code table
pub const CODEPAGE_WCP1255: u8 = 33;
/// Arabic character code table
pub const CODEPAGE_WCP1256: u8 = 34;
/// Vietnam character code table
pub const CODEPAGE_WCP1258: u8 = 35;
/// Latin 2 character code table
pub const CODEPAGE_ISO_8859_2: u8 = 36;
/// Latin 3 character code table
pub const CODEPAGE_ISO_8859_3: u8 = 37;
/// Baltic character code table
pub const CODEPAGE_ISO_8859_4: u8 = 38;
/// Cyrillic character code table
pub const CODEPAGE_ISO_8859_5: u8 = 39;
/// Arabic character code table
pub const CODEPAGE_ISO_8859_6: u8 = 40;
/// Greek character code table
pub const CODEPAGE_ISO_8859_7: u8 = 41;
/// Hebrew character code table
pub const CODEPAGE_ISO_8859_8: u8 = 42;
/// Turkish character code table
pub const CODEPAGE_ISO_8859_9: u8 = 43;
/// Latin 3 character code table
pub const CODEPAGE_ISO_8859_15: u8 = 44;
/// Thai 2 character code page
pub const CODEPAGE_THAI2: u8 = 45;
/// Hebrew character code page
pub const CODEPAGE_CP856: u8 = 46;
/// Thai character code page
pub const CODEPAGE_CP874: u8 = 47;

// ASCII character set
pub const ASCII_ESC: u8 = 0x1B;
pub const ASCII_DC2: u8 = 0x12;
pub const ASCII_GS: u8 = 0x1D;
pub const ASCII_TAB: u8 = 0x09;
