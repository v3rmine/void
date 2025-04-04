/****************************************************************************
 * ssd1306.h
 *
 * I2C Driver for 128x64 oled display drivers
 *
 * Copyright 2016 Nathael Pajani <nathael.pajani@ed3l.fr>
 * Copyright 2022 Chomienne Anthony <anthony@mob-dev.fr>
 *
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 *************************************************************************** */

/**
 * This code is mainly an adaptation of code provided by Techno-Innov (Nathael Pajani <nathael.pajani@ed3l.fr>)
 * www.techno-innov.fr
 * http://git.techno-innov.fr
 */

#ifndef SSD1306
#define SSD1306

#define SSD130x_ADDR 0x7A

#define SSD130x_NB_LINES 64
#define SSD130x_NB_PAGES 8
#define SSD130x_NB_COL   128


enum ssd130x_defs {
	SSD130x_DISP_OFF = 0,
	SSD130x_DISP_ON,
	SSD130x_DISP_RAM,
	SSD130x_DISP_BLANK,
	/* For reverse video */
	SSD130x_DISP_NORMAL,
	SSD130x_DISP_REVERSE,
	/* For scan dirrection */
	SSD130x_SCAN_TOP_BOTTOM,
	SSD130x_SCAN_BOTTOM_TOP,
	SSD130x_RIGHT_TO_LEFT,
	SSD130x_LEFT_TO_RIGHT,
	/* For display offset */
	SSD130x_MOVE_TOP,
	SSD130x_MOVE_BOTTOM,
};


#define SSD130x_DATA_ONLY       0x40
#define SSD130x_NEXT_BYTE_DATA  0xC0
#define SSD130x_NEXT_BYTE_CMD   0x80

/* Display controll */
#define SSD130x_CMD_CONTRAST           0x81
#define SSD130x_CMD_DISP_RAM           0xA4
#define SSD130x_CMD_DISP_NORAM         0xA5
#define SSD130x_CMD_DISP_NORMAL        0xA6
#define SSD130x_CMD_DISP_REVERSE       0xA7
#define SSD130x_CMD_DISP_OFF           0xAE
#define SSD130x_CMD_DISP_ON            0xAF

/* Scrolling controll */
#define SSD130x_CMD_SCROLL_RIGHT       0x26
#define SSD130x_CMD_SCROLL_LEFT        0x27
#define SSD130x_CMD_VSCROLL_RIGHT      0x29
#define SSD130x_CMD_VSCROLL_LEFT       0x2A
#define SSD130x_CMD_STOP_SCROLL        0x2E
#define SSD130x_CMD_START_SCROLL       0x2F
#define SSD130x_CMD_VSCROLL_REGION     0xA3
/* Data bytes for scrolling controll */
#define SSD130x_SCROLL_DATA_DUMMY  0x00
#define SSD130x_SCROLL_DATA_END    0xFF
#define SSD130x_SCROLL_DATA_START_PAGE(x) ((x) & 0x07)
#define SSD130x_SCROLL_DATA_END_PAGE(x)   ((x) & 0x07)
#define SSD130x_SCROLL_DATA_ROWS(x)   ((x) & 0x3F)
#define SSD130x_SCROLL_DATA_STEP(x) ((x) & 0x07)
/* Scroll steps definitions */
#define SSD130x_SCROLL_2_FRAMES    0x07
#define SSD130x_SCROLL_3_FRAMES    0x04
#define SSD130x_SCROLL_4_FRAMES    0x05
#define SSD130x_SCROLL_5_FRAMES    0x00
#define SSD130x_SCROLL_25_FRAMES   0x06
#define SSD130x_SCROLL_64_FRAMES   0x01
#define SSD130x_SCROLL_128_FRAMES  0x02
#define SSD130x_SCROLL_256_FRAMES  0x03

/* GDDRAM Adressing */
#define SSD130x_CMD_ADDR_MODE          0x20
/* Data bytes for adressing mode election */
#define SSD130x_ADDR_TYPE_HORIZONTAL 0x00
#define SSD130x_ADDR_TYPE_VERTICAL   0x01
#define SSD130x_ADDR_TYPE_PAGE       0x02

/* GDDRAM Page adressing mode */
#define SSD130x_CMD_COL_LOW_NIBLE(x)   (0x00 + ((x) & 0x0F))
#define SSD130x_CMD_COL_HIGH_NIBLE(x)  (0x10 + ((x) & 0x0F))
#define SSD130x_CMD_PAGE_START_ADDR(x) (0xB0 + ((x) & 0x07))

/* GDDRAM horizontal or vertical addressing mode */
#define SSD130x_CMD_COL_ADDR           0x21
#define SSD130x_CMD_PAGE_ADDR          0x22
/* Data bytes for horizontal or vertical adressing mode */
#define SSD130x_ADDR_COL(x)       ((x) & 0x7F)
#define SSD130x_ADDR_PAGE(x)      ((x) & 0x07)

/* Charge pump */
#define SSD130x_EXT_VCC       0x01
#define SSD130x_INTERNAL_PUMP 0x00
#define SSD130x_CMD_CHARGE_PUMP    0x8D
#define SSD130x_CMD_CHARGE_EXT     0x10
#define SSD130x_CMD_CHARGE_INTERN  0x14

/* Hardware configuration */
#define SSD130x_CMD_START_LINE(x)      (0x40 + ((x) & 0x3F))
#define SSD130x_CMD_SEG0_MAP_RIGHT     0xA1
#define SSD130x_CMD_SEG0_MAP_LEFT      0xA0

/* Hardware configuration : Mux ratio */
#define SSD130x_CMD_SET_MUX            0xA8
/* Set mux ratio Data to N+1 (Values for N from 0 to 14 are invalid) */
#define SSD130x_MUX_DATA(x)       ((x) & 0x3F)  /* Reset is N=63 (64 mux) */

/* Hardware configuration : COM Scan */
#define SSD130x_CMD_COM_SCAN_NORMAL    0xC0  /* Reset mode : top to bottom */
#define SSD130x_CMD_COM_SCAN_REVERSE   0xC8  /* Bottom to top */
#define SSD130x_CMD_DISPLAY_OFFSET     0xD3
/* Data for display offset (COM shift) */
#define SSD130x_OFFSET_DATA(x)    ((x) & 0x3F)
#define SSD130x_CMD_COM_PIN_CONF       0xDA
/* Data for COM pins hardware configuration */
#define SSD130x_COM_SEQUENTIAL    (0x00 << 4)
#define SSD130x_COM_ALTERNATIVE   (0x01 << 4) /* Reset mode */
#define SSD130x_COM_NO_REMAP      (0x00 << 5) /* Reset mode */
#define SSD130x_COM_REMAP         (0x01 << 5)

/* Timing and driving scheme : Clock */
#define SSD130x_CMD_DISP_CLK_DIV       0xD5
#define SSD130x_CLK_DIV(x)        ((x) & 0x0F)  /* Set to N+1 (Default is 0+1) */
#define SSD130x_CLK_FREQ(x)       (((x) & 0x0F) << 4)  /* Reset is 0x80 */

/* Timing and driving scheme : Precharge */
#define SSD130x_CMD_SET_PRECHARGE      0xD9
#define SSD130x_PRECHARGE_PHASE1(x)    ((x) & 0x0F)  /* Default to 2, 0 is invalid */
#define SSD130x_PRECHARGE_PHASE2(x)    (((x) & 0x0F) << 4) /* Default to 2, 0 is invalid */

/* Timing and driving scheme : Voltage */
#define SSD130x_CMD_VCOM_LEVEL         0xDB
#define SSD130x_VCOM_065    0x00
#define SSD130x_VCOM_077    0x20
#define SSD130x_VCOM_083    0x30

/* NO-OP */
#define SSD130x_CMD_NOP                0xE3

/* Status register read */
#define SSD130x_STATUS_ON     (0x01 << 6)

#define GDDRAM_SIZE   (128 * 8)

#define OLED_LINE_CHAR_LENGTH     (SSD130x_NB_COL / 8)
#define DISPLAY_LINE_LENGTH  (OLED_LINE_CHAR_LENGTH + 1)

#include <cstdint>
#include <MicroBit.h>

class ssd1306 {
    public:

        /**
         *
         */
        ssd1306(MicroBit* uB, MicroBitI2C* uBi2c, MicroBitPin* pin_reset,uint8_t addr = SSD130x_ADDR);

        /**
         * Power Off the screen
         */
        int power_off();

        /**
         * Power On the screen
         */
        int power_on();

        /**
         * Display a single char a position (line, col)
         */
        void display_char(uint8_t line, uint8_t col, uint8_t c);

        /**
         * Display a text at position (line, col)
         */
        int display_line(uint8_t line, uint8_t col, const char* text);

        /**
         * update screen display
         * should be called after a series of display change
         */
        int update_screen();

    private:
        int initialize();
        int send_command(uint8_t cmd, uint8_t* data, uint8_t len);
        int set_mem_addressing_mode(uint8_t mode);
        int set_column_address(uint8_t col_start, uint8_t col_end);
        int set_page_address(uint8_t page_start, uint8_t page_end);
        int set_display_on(uint8_t use_ram);
        int display_power(uint8_t status);
        int set_scan_direction();
        int set_read_direction();
        int set_display_offset(uint8_t dir, uint8_t nb_lines);
        int set_mux_ratio(uint8_t ratio);
        int set_display_clock(uint8_t divide, uint8_t frequency);
        int set_contrast(uint8_t contrast);
        int display_video_reverse();

        int buffer_set(uint8_t *gddram, uint8_t val);
        int buffer_set_pixel(uint8_t* gddram, uint8_t x0, uint8_t y0, uint8_t state);
        int buffer_set_tile(uint8_t* gddram, uint8_t x0, uint8_t y0, uint8_t* tile);


        MicroBit* uBit;
        MicroBitI2C* i2c;
        MicroBitPin* reset;
        uint8_t gddram[ 1 + GDDRAM_SIZE ];
        uint8_t address;
        uint8_t video_mode;
        uint8_t contrast;
        uint8_t scan_dir;
        uint8_t read_dir;
        uint8_t offset_dir;
        uint8_t offset;
        uint8_t charge_pump;
        bool fullscreen;

};

#endif
