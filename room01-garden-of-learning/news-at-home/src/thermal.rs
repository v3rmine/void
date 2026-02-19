use std::io::{Bytes, Read, Write};

use esp_idf_svc::hal::delay::FreeRtos;

use crate::{
    constants::{
        ASCII_DC2, ASCII_ESC, ASCII_FF, ASCII_GS, ASCII_TAB, BOLD_MASK, DOUBLE_HEIGHT_MASK,
        DOUBLE_WIDTH_MASK, FONT_MASK, INVERSE_MASK, PRINTER_MAX_COLUMNS, STRIKE_MASK, UPDOWN_MASK,
    },
    helpers::get_next_byte,
};

/// SOURCE:
/// <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.h#L339>
#[derive(Debug, Default)]
pub struct ThermalInterface<S: Write + Read> {
    stream: S,
    print_mode: u8,
    /// Last character issued to printer
    prev_byte: u8,
    /// Last horizontal column printed
    column: u8,
    /// Page width (output 'wraps' at this point)
    max_column: u8,
    /// Height of characters, in 'dots'
    char_height: u8,
    /// Inter-line spacing (not line height), in dots
    line_spacing: u8,
    /// Barcode height in dots, not including text
    barcode_height: u8,
    max_chunk_height: u8,
    /// Firmware version
    firmware: u16,
    /// True if DTR pin set & printer initialized
    dtr_enabled: bool,
    /// Wait until micros() exceeds this before sending byte
    resume_time: i64,
    /// Time to print a single dot line, in microseconds
    dot_print_time: u32,
    /// Time to feed a single dot line, in microseconds
    dot_feed_time: u32,
    baudrate: u32,
}

/// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.h#L78>
pub enum BarcodeType {
    /// UPC-A barcode system. 11-12 char
    UpcA,
    /// UPC-E barcode system. 11-12 char
    UpcE,
    /// EAN13 (JAN13) barcode system. 12-13 char
    Ean13,
    /// EAN8 (JAN8) barcode system. 7-8 char
    Ean8,
    /// CODE39 barcode system. 1<=num of chars
    Code39,
    /// ITF barcode system. 1<=num of chars, must be an even number
    Itf,
    /// CODABAR barcode system. 1<=num<=255
    Codabar,
    /// CODE93 barcode system. 1<=num<=255
    Code93,
    /// CODE128 barcode system. 2<=num<=255
    Code128,
    /// Unknown barcode type
    Unknown(u8),
}

impl From<u8> for BarcodeType {
    fn from(value: u8) -> Self {
        match value {
            0 => BarcodeType::UpcA,
            1 => BarcodeType::UpcE,
            2 => BarcodeType::Ean13,
            3 => BarcodeType::Ean8,
            4 => BarcodeType::Code39,
            5 => BarcodeType::Itf,
            6 => BarcodeType::Codabar,
            7 => BarcodeType::Code93,
            8 => BarcodeType::Code128,
            // Default to Unknown
            value => BarcodeType::Unknown(value),
        }
    }
}

// Return microseconds elapsed since boot
fn micros() -> i64 {
    unsafe { esp_idf_svc::hal::sys::esp_timer_get_time() }
}

impl<S: Default + Write + Read> ThermalInterface<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            baudrate: 19200,
            dtr_enabled: false,
            ..Default::default()
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L67C19-L67C67>
    pub fn byte_time(&self) -> u32 {
        ((11 * 1000000) + (self.baudrate / 2)) / self.baudrate
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L76C24-L76C34>
    pub fn timeout_set(&mut self, timeout: i64) {
        if !self.dtr_enabled {
            self.resume_time = micros() + timeout;
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L82>
    pub fn timeout_wait(&self) {
        if self.dtr_enabled {
            FreeRtos::delay_ms(1);
        } else {
            FreeRtos::delay_ms((micros() - self.resume_time) as u32);
        }
    }

    /// Printer performance may vary based on the power supply voltage,
    /// thickness of paper, phase of the moon and other seemingly random
    /// variables.  This method sets the times (in microseconds) for the
    /// paper to advance one vertical 'dot' when printing and when feeding.
    /// For example, in the default initialized state, normal-sized text is
    /// 24 dots tall and the line spacing is 30 dots, so the time for one
    /// line to be issued is approximately 24 * print time + 6 * feed time.
    /// The default print and feed times are based on a random test unit,
    /// but as stated above your reality may be influenced by many factors.
    /// This lets you tweak the timing to avoid excessive delays and/or
    /// overrunning the printer buffer.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/master/Adafruit_Thermal.cpp#L8>
    pub fn set_times(&mut self, print_time: u32, feed_time: u32) {
        self.dot_print_time = print_time;
        self.dot_feed_time = feed_time;
    }

    /// The next helper are used when issuing configuration
    /// commands, printing bitmaps or barcodes, etc. Not when printing text.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.timeout_wait();
        self.stream
            .write_all(&bytes)
            .expect("failed to write bytes to output stream");
        self.timeout_set(bytes.len() as i64 * self.byte_time() as i64);
    }

    /// The underlying method for all high-level printing (e.g. println()).
    /// The inherited Print class handles the rest!
    pub fn write(&mut self, mut char_to_write: u8) {
        self.timeout_wait();
        self.stream
            .write(&[char_to_write])
            .expect("failed to write char to output stream");
        let mut d = self.byte_time();
        if char_to_write == b'\n' || self.column >= self.max_column {
            // If newline or wrap;
            d += if self.prev_byte == b'\n' {
                // Feed line
                (self.char_height + self.line_spacing) as u32 * self.dot_feed_time
            } else {
                // Text line
                (self.char_height as u32 * self.dot_print_time)
                    + (self.line_spacing as u32 * self.dot_feed_time)
            };

            self.column = 0;
            char_to_write = b'\n';
        } else {
            self.column += 1;
        }

        self.timeout_set(d as i64);
        self.prev_byte = char_to_write;
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L168>
    pub fn begin(&mut self, version: u16) {
        self.firmware = version;

        // The printer can't start receiving data immediately upon power up --
        // it needs a moment to cold boot and initialize.  Allow at least 1/2
        // sec of uptime before printer can receive data.
        self.timeout_set(500000);
    }

    /// SOURCE:
    /// <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L195>
    pub fn reset(&mut self) {
        // Init command
        self.write_bytes(&[ASCII_ESC, b'@']);
        self.prev_byte = b'\n';
        self.column = 0;
        self.max_column = 32;
        self.char_height = 24;
        self.line_spacing = 6;
        self.barcode_height = 50;

        if self.firmware >= 264 {
            // Configure tab stops on recent printers
            // Set tab stops...
            self.write_bytes(&[ASCII_ESC, b'D']);
            // ...every 4 columns
            self.write_bytes(&[4, 8, 12, 16]);
            // 0 marks end-of-line
            self.write_bytes(&[20, 24, 28, 0]);
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L232>
    pub fn test_page(&mut self) {
        self.write_bytes(&[ASCII_DC2, b'T']);
        // 26 lines w/text (ea. 24 dots high)
        // 26 text lines (feed 6 dots) + blank line
        self.timeout_set(
            (self.dot_print_time * 24 * 26 + self.dot_feed_time * (6 + 26 + 30)) as i64,
        );
    }

    /// Default is 50
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L239C6-L244C2>
    pub fn set_barcode_height(&mut self, initial_height: u8) {
        let height = if initial_height < 1 {
            1
        } else {
            initial_height
        };
        self.barcode_height = height;
        self.write_bytes(&[ASCII_GS, b'h', height]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L246>
    pub fn print_barcode(&mut self, content: &str, barcode_type: impl Into<u8>) {
        let mut barcode_type = barcode_type.into();

        // TOCHECK: Recent firmware can't print barcode w/o feed first???
        self.feed(1);
        if self.firmware >= 264 {
            barcode_type += 65;
        }

        // Print label below barcode
        self.write_bytes(&[ASCII_GS, b'H', 2]);
        // Barcode width 3 (0.375/1.0mm thin/thick)
        self.write_bytes(&[ASCII_GS, b'w', 3]);
        // Barcode type (listed in .h file)
        self.write_bytes(&[ASCII_GS, b'k', barcode_type]);

        if self.firmware >= 264 {
            // In rust `as u8` overflow to 255
            let len = content.len() as u8;
            self.write_bytes(&[len]);
            let text = content.as_bytes();
            for i in 0..len as usize {
                self.write_bytes(&[text[i]]);
            }
        } else {
            let text = content.as_bytes();
            for &byte in text {
                self.write_bytes(&[byte]);
            }
            // In rust, strings are not null-terminated by default
            self.write_bytes(&[0]);
        }

        self.timeout_set(((self.barcode_height + 40) as u32 * self.dot_print_time) as i64);
    }

    // SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L281>
    pub fn adjust_char_values(&mut self, print_mode: u8) {
        let mut char_width: u32;

        if (print_mode & FONT_MASK) > 0 {
            // FontB
            self.char_height = 17;
            char_width = 9;
        } else {
            // FontA
            self.char_height = 24;
            char_width = 12;
        }

        // Double Width Mode
        if (print_mode & DOUBLE_WIDTH_MASK) > 0 {
            self.max_column /= 2;
            char_width *= 2;
        }
        // Double Height Mode
        if (print_mode * DOUBLE_HEIGHT_MASK) > 0 {
            self.char_height *= 2;
        }

        self.max_column = (PRINTER_MAX_COLUMNS / char_width) as u8;
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L304>
    pub fn set_print_mode(&mut self, mask: u8) {
        self.print_mode |= mask;
        self.write_print_mode();
        self.adjust_char_values(self.print_mode);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L312>
    pub fn unset_print_mode(&mut self, mask: u8) {
        self.print_mode &= !mask;
        self.write_print_mode();
        self.adjust_char_values(self.print_mode);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L320>
    pub fn write_print_mode(&mut self) {
        self.write_bytes(&[ASCII_ESC, b'!', self.print_mode]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L324>
    pub fn normal(&mut self) {
        self.print_mode = 0;
        self.write_print_mode();
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L329>
    pub fn inverse_on(&mut self) {
        if self.firmware >= 268 {
            self.write_bytes(&[ASCII_GS, b'B', 1]);
        } else {
            self.set_print_mode(INVERSE_MASK);
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L337>
    pub fn inverse_off(&mut self) {
        if self.firmware >= 268 {
            self.write_bytes(&[ASCII_GS, b'B', 0]);
        } else {
            self.unset_print_mode(INVERSE_MASK);
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L345>
    pub fn upside_down_on(&mut self) {
        if self.firmware >= 268 {
            self.write_bytes(&[ASCII_ESC, b'{', 1]);
        } else {
            self.set_print_mode(UPDOWN_MASK);
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L353>
    pub fn upside_down_off(&mut self) {
        if self.firmware >= 268 {
            self.write_bytes(&[ASCII_ESC, b'{', 0]);
        } else {
            self.unset_print_mode(UPDOWN_MASK);
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L361>
    pub fn double_height_on(&mut self) {
        self.set_print_mode(DOUBLE_HEIGHT_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L363>
    pub fn double_height_off(&mut self) {
        self.unset_print_mode(DOUBLE_HEIGHT_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L365>
    pub fn double_width_on(&mut self) {
        self.set_print_mode(DOUBLE_WIDTH_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L367>
    pub fn double_width_off(&mut self) {
        self.unset_print_mode(DOUBLE_WIDTH_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L369>
    pub fn strike_on(&mut self) {
        self.set_print_mode(STRIKE_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L371>
    pub fn strike_off(&mut self) {
        self.unset_print_mode(STRIKE_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L373>
    pub fn bold_on(&mut self) {
        self.set_print_mode(BOLD_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L375>
    pub fn bold_off(&mut self) {
        self.unset_print_mode(BOLD_MASK);
    }
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L377>
    pub fn justify(&mut self, value: char) {
        let pos: u8;

        match value.to_ascii_uppercase() {
            'C' => {
                pos = 1;
            }
            'R' => {
                pos = 2;
            }
            'L' | _ => {
                pos = 0;
            }
        }

        self.write_bytes(&[ASCII_ESC, b'a', pos]);
    }

    /// Feeds by the specified number of lines
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L396>
    pub fn feed(&mut self, mut lines_count: u8) {
        if self.firmware >= 264 {
            self.write_bytes(&[ASCII_ESC, b'd', lines_count]);
            self.timeout_set((self.dot_feed_time * self.char_height as u32) as i64);
            self.prev_byte = b'\n';
            self.column = 0;
        } else {
            // Feed manually; old firmware feeds excess lines
            while lines_count > 0 {
                self.write(b'\n');
                lines_count -= 1;
            }
        }
    }

    /// Feeds by the specified number of individual pixel rows
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L409>
    pub fn feed_rows(&mut self, rows_count: u8) {
        self.write_bytes(&[ASCII_ESC, b'J', rows_count]);
        self.timeout_set((rows_count as u32 * self.dot_feed_time) as i64);
        self.prev_byte = b'\n';
        self.column = 0;
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L416>
    pub fn flush(&mut self) {
        self.write_bytes(&[ASCII_FF]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L418>
    pub fn set_size(&mut self, value: char) {
        match value.to_ascii_uppercase() {
            // Large: double width and height
            'L' => {
                self.double_width_on();
                self.double_height_on();
            }
            // Medium: double height
            'M' => {
                self.double_width_off();
                self.double_height_on();
            }
            // Small: standard width and height
            _ => {
                self.double_width_off();
                self.double_height_off();
            }
        }
    }

    /// ESC 7 n1 n2 n3 Setting Control Parameter Command
    /// n1 = "max heating dots" 0-255 -- max number of thermal print head
    ///      elements that will fire simultaneously.  Units = 8 dots (minus 1).
    ///      Printer default is 7 (64 dots, or 1/6 of 384-dot width), this code
    ///      sets it to 11 (96 dots, or 1/4 of width).
    /// n2 = "heating time" 3-255 -- duration that heating dots are fired.
    ///      Units = 10 us.  Printer default is 80 (800 us), this code sets it
    ///      to value passed (default 120, or 1.2 ms -- a little longer than
    ///      the default because we've increased the max heating dots).
    /// n3 = "heating interval" 0-255 -- recovery time between groups of
    ///      heating dots on line; possibly a function of power supply.
    ///      Units = 10 us.  Printer default is 2 (20 us), this code sets it
    ///      to 40 (throttled back due to 2A supply).
    /// More heating dots = more peak current, but faster printing speed.
    /// More heating time = darker print, but slower printing speed and
    /// possibly paper 'stiction'.  More heating interval = clearer print,
    /// but slower printing speed.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L466>
    pub fn set_heat_config(&mut self, dots: u8, time: u8, interval: u8) {
        // Esc 7 (print settings)
        self.write_bytes(&[ASCII_ESC, 7]);
        // Heating dots, heat time, heat interval
        self.write_bytes(&[dots, time, interval]);
    }

    /// Print density description from manual:
    /// DC2 # n Set printing density
    /// D4..D0 of n is used to set the printing density.  Density is
    /// 50% + 5% * n(D4-D0) printing density.
    /// D7..D5 of n is used to set the printing break time.  Break time
    /// is n(D7-D5)*250us.
    /// (Unsure of the default value for either -- not documented)
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L479>
    pub fn set_print_density(&mut self, density: u8, break_time: u8) {
        self.write_bytes(&[ASCII_DC2, b'#', (density << 5) | break_time]);
    }

    /// Underlines of different weights can be produced:
    /// 0 - no underline
    /// 1 - normal underline
    /// 2 - thick underline
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L487>
    pub fn underline_on(&mut self, mut weight: u8) {
        if weight > 2 {
            weight = 2;
        }
        self.write_bytes(&[ASCII_ESC, b'-', weight]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L493>
    pub fn underline_off(&mut self) {
        self.write_bytes(&[ASCII_ESC, b'-', 0]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L495>
    pub fn print_bitmap_from_slice(&mut self, width: u16, height: u16, bitmap: &[u8]) {
        // Create a cursor over the slice to treat it as a Read stream
        let bitmap_stream = std::io::Cursor::new(bitmap);
        self.print_bitmap(width, height, bitmap_stream);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L534C24-L534C35>
    pub fn print_bitmap(&mut self, width: u16, height: u16, mut stream: impl Read) {
        // Round up to next byte boundary
        let row_bytes = (width as u8 + 7) / 8;
        // 384 pixels max width
        let row_bytes_clipped: u8 = if row_bytes >= 48 { 48 } else { row_bytes as u8 };

        // Est. max rows to write at once, assuming 256 byte printer buffer.
        let chunk_height_limit = if self.dtr_enabled {
            // Buffer doesn't matter, handshake!
            255u8
        } else {
            let chunk_height_limit = (256u16 / row_bytes_clipped as u16) as u8;
            if chunk_height_limit > self.max_chunk_height {
                self.max_chunk_height
            } else if chunk_height_limit < 1 {
                1
            } else {
                chunk_height_limit
            }
        };

        let mut chunk_height;
        let mut row_start = 0;
        while row_start < height {
            // Issue up to chunkHeightLimit rows at a time:
            chunk_height = height - row_start;
            if chunk_height > chunk_height_limit as u16 {
                chunk_height = chunk_height_limit as u16;
            }

            self.write_bytes(&[ASCII_DC2, b'*', chunk_height as u8, row_bytes_clipped as u8]);

            for _y in 0..chunk_height {
                for _x in 0..row_bytes_clipped {
                    let c = get_next_byte(&mut stream).expect("failed to get next byte in stream");
                    self.timeout_wait();
                    self.stream
                        .write(&[c])
                        .expect("failed to write to output stream");
                }

                // Discard bytes
                let bytes_to_discard = row_bytes.saturating_sub(row_bytes_clipped);
                for _i in 0..bytes_to_discard {
                    let _c = get_next_byte(&mut stream).expect("failed to get next byte in stream");
                }
            }

            self.timeout_set(((chunk_height as u32) * self.dot_print_time) as i64);

            row_start += chunk_height_limit as u16;
        }

        self.prev_byte = b'\n';
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L577>
    pub fn print_bitmap_from_stream(&mut self, mut stream: impl Read) {
        let mut tmp: u8;
        let (width, height): (u16, u16);

        tmp = get_next_byte(&mut stream).expect("empty stream");
        width = ((get_next_byte(&mut stream).expect("no next byte in stream for width") as u16)
            << 8)
            + tmp as u16;

        tmp = get_next_byte(&mut stream).expect("no next byte in stream for height");
        height = ((get_next_byte(&mut stream).expect("no next byte in stream for height") as u16)
            << 8)
            + tmp as u16;

        self.print_bitmap(width, height, stream);
    }

    /// Take the printer offline. Print commands sent after this will be ignored until 'online' is called.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L592>
    pub fn offline(&mut self) {
        self.write_bytes(&[ASCII_ESC, b'=', 0]);
    }

    /// Take the printer back online. Subsequent print commands will be obeyed.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L595>
    pub fn online(&mut self) {
        self.write_bytes(&[ASCII_ESC, b'=', 1]);
    }

    /// Put the printer into a low-energy state immediately.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L597C4-L597C56>
    pub fn sleep(&mut self) {
        // Can't be 0, that means 'don't sleep'
        self.sleep_after(1);
    }

    /// Put the printer into a low-energy state after the given number of seconds.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L604>
    pub fn sleep_after(&mut self, seconds: u16) {
        if self.firmware >= 264 {
            self.write_bytes(&[ASCII_ESC, b'8', (seconds) as u8, (seconds >> 8) as u8]);
        } else {
            self.write_bytes(&[ASCII_ESC, b'8', seconds as u8]);
        }
    }

    /// Wake the printer from a low-energy state.
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L613>
    pub fn wake(&mut self) {
        // Reset timeout counter
        self.timeout_set(0);
        // Wake
        self.write_bytes(&[255]);

        if self.firmware >= 264 {
            FreeRtos::delay_ms(50);
            // Sleep off (important!)
            self.write_bytes(&[ASCII_ESC, b'8', 0, 0]);
        } else {
            // Datasheet recommends a 50ms delay before issuing further commands,
            // but in practice this alone isn't sufficient (e.g. text size/style
            // commands may still be misinterpreted on wake). A slightly longer
            // delay, interspersed with NUL chars (no-ops) seems to help.
            for _ in 0..10 {
                self.write_bytes(&[0]);
                self.timeout_set(10_000);
            }
        }
    }

    /// Check the status of the paper using the printer's self reporting ability.
    /// Might not work on all printers!
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L634>
    pub fn has_paper(&mut self) -> bool {
        if self.firmware >= 264 {
            self.write_bytes(&[ASCII_ESC, b'v', 0]);
        } else {
            self.write_bytes(&[ASCII_GS, b'r', 0]);
        }

        let mut status: i16 = -1;
        for _ in 0..10 {
            if let Some(next_byte) = get_next_byte(&mut self.stream) {
                status = next_byte as i16;
            }
            FreeRtos::delay_ms(100);
        }

        0 == (status & 0b00000100)
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L653>
    pub fn set_line_height(&mut self, initial_line_height: u8) {
        let line_height = if initial_line_height < 24 {
            24
        } else {
            initial_line_height
        };
        self.line_spacing = line_height - 24;

        // The printer doesn't take into account the current text height
        // when setting line height, making this more akin to inter-line
        // spacing.  Default line spacing is 30 (char height of 24, line
        // spacing of 6).
        self.write_bytes(&[ASCII_ESC, b'3', line_height]);
    }

    pub fn set_max_chunk_height(&mut self, max_chunk_height: u8) {
        self.max_chunk_height = max_chunk_height;
    }

    /// Alters some chars in ASCII 0x23-0x7E range; see datasheet
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L670>
    pub fn set_charset(&mut self, initial_charset: u8) {
        let charset = if initial_charset > 15 {
            15
        } else {
            initial_charset
        };
        self.write_bytes(&[ASCII_ESC, b'R', charset]);
    }

    /// Selects alt symbols for 'upper' ASCII values 0x80-0xFF
    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L677>
    pub fn set_code_page(&mut self, initial_code_page: u8) {
        let code_page = if initial_code_page > 47 {
            47
        } else {
            initial_code_page
        };
        self.write_bytes(&[ASCII_ESC, b't', code_page]);
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L683>
    pub fn tab(&mut self) {
        self.write_bytes(&[ASCII_TAB]);
        self.column = (self.column + 4) & 0b11111100;
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L688>
    pub fn set_font(&mut self, font: char) {
        match font.to_ascii_uppercase() {
            'B' => {
                self.set_print_mode(FONT_MASK);
            }
            'A' | _ => {
                self.unset_print_mode(FONT_MASK);
            }
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L699>
    pub fn set_char_spacing(&mut self, spacing: u8) {
        self.write_bytes(&[ASCII_ESC, b' ', spacing]);
    }
}
