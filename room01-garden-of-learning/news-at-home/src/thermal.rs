use std::ptr::write_bytes;

use esp_idf_svc::{
    hal::delay::FreeRtos,
    io::{Read, Write},
};

use crate::constants::{ASCII_DC2, ASCII_ESC, ASCII_GS, ASCII_TAB};

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
    resume_time: u32,
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
}

// TODO: Return microseconds elapsed since boot
fn micros() -> u32 {
    0
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
    pub fn timeout_set(&mut self, timeout: u32) {
        if !self.dtr_enabled {
            self.resume_time = micros() + timeout;
        }
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L82>
    pub fn timeout_wait(&self) {
        if self.dtr_enabled {
            // TODO:
        } else {
            // TODO:
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
        for byte in bytes {
            self.stream.write(&[*byte]);
        }
        self.timeout_set(bytes.len() as u32 * self.byte_time());
    }

    /// The underlying method for all high-level printing (e.g. println()).
    /// The inherited Print class handles the rest!
    pub fn write(&mut self, char_to_write: &mut u8) {
        self.timeout_wait();
        self.stream.write(&[*char_to_write]);
        let mut d = self.byte_time();
        if *char_to_write == b'\n' || self.column >= self.max_column {
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
            *char_to_write = b'\n';
        } else {
            self.column += 1;
        }

        self.timeout_set(d);
        self.prev_byte = *char_to_write;
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
        self.timeout_set(self.dot_print_time * 24 * 26 + self.dot_feed_time * (6 + 26 + 30));
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
    pub fn print_barcode(&mut self, content: &str, barcode_type: BarcodeType) {
        // TODO:
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L495>
    pub fn print_bitmap_from_slice(&mut self, width: u8, height: u8, bitmap: &[u8]) {
        // TODO:
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L534C24-L534C35>
    pub fn print_bitmap(&mut self, width: u8, height: u8, stream: impl Read) {
        // Round up to next byte boundary
        let row_bytes = (width + 7) / 8;
        // 384 pixels max width
        let row_bytes_clipped = if row_bytes >= 48 { 48 } else { row_bytes };

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

        let mut chunk_height = 0;
        let mut row_start = 0;
        while row_start < height {
            // Issue up to chunkHeightLimit rows at a time:
            chunk_height = height - row_start;
            if chunk_height > chunk_height_limit {
                chunk_height = chunk_height_limit;
            }

            self.write_bytes(&[ASCII_DC2, b'*', chunk_height, row_bytes_clipped]);

            for y in 0..chunk_height {
                for x in 0..row_bytes_clipped {
                    // TODO: WHILE READ
                    self.timeout_wait();
                    // TODO: WRITE
                }
                for i in (row_bytes - row_bytes_clipped)..0 {
                    // TODO: WHILE READ
                }
            }

            self.timeout_set((chunk_height as u32) * self.dot_print_time);

            row_start += chunk_height_limit;
        }

        self.prev_byte = b'\n';
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L577>
    pub fn print_bitmap_from_stream(&mut self, stream: impl Read) {
        // TODO:
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
    pub fn sleep_after(&mut self, seconds: u8) {
        if self.firmware >= 264 {
            // TODO: Replace this unsafe operation
            #[allow(arithmetic_overflow)]
            self.write_bytes(&[ASCII_ESC, b'8', seconds, seconds >> 8]);
            self.write_bytes(&[ASCII_ESC, b'8', seconds]);
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
        if (self.firmware >= 264) {
            self.write_bytes(&[ASCII_ESC, b'v', 0]);
        } else {
            self.write_bytes(&[ASCII_GS, b'r', 0]);
        }

        let mut status = -1;
        for _ in 0..10 {
            // TODO: read status
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
    pub fn set_font(font: char) {
        // TODO:
    }

    /// SOURCE: <https://github.com/adafruit/Adafruit-Thermal-Printer-Library/blob/54786351af1d84580c4ae555d439756679b0dc44/Adafruit_Thermal.cpp#L699>
    pub fn set_char_spacing(&mut self, spacing: u8) {
        self.write_bytes(&[ASCII_ESC, b' ', spacing]);
    }
}
