#include "MicroBit.h"
#include <errno.h>

#include "ssd1306.h"
#include "font.h"

#define ROW(x)   VERTICAL_REV(x)
DECLARE_FONT(font);

ssd1306::ssd1306(MicroBit* uB, MicroBitI2C* uBi2c, MicroBitPin* pin_reset, uint8_t addr):uBit(uB),i2c(uBi2c),reset(pin_reset), address(addr)
{
    buffer_set(gddram, 0x00);
    video_mode = SSD130x_DISP_NORMAL;
    contrast = 128;
    scan_dir = SSD130x_SCAN_BOTTOM_TOP;
    read_dir = SSD130x_RIGHT_TO_LEFT;
    offset_dir = SSD130x_MOVE_TOP;
    offset = 4;
    fullscreen = true;
    charge_pump = SSD130x_INTERNAL_PUMP;
    initialize();
}

int ssd1306::initialize()
{
    int ret = 0;
    uint8_t val = 0;
    fullscreen = 1;

    reset->setDigitalValue(1);
    uBit->sleep(1);
    reset->setDigitalValue(0);
    uBit->sleep(10);
    reset->setDigitalValue(1);

    ret = display_power(SSD130x_DISP_OFF);//AE
    if (ret != 0)
    {
        uBit->display.scroll("Power Error");
        return ret;
    }

    ret = set_display_on(SSD130x_DISP_RAM);//A4
    if (ret != 0)
    {
        uBit->display.scroll("DISP RAM Error");
        return ret;
    }
    //0xD5 0xF0
    set_display_clock(0x00,0x0F);

    //0xA8 0x3F
    set_mux_ratio(0x3F);
    //0xD3 0x00
    set_display_offset(offset_dir, offset);
    //0|0x00
    send_command(SSD130x_CMD_COL_LOW_NIBLE(0x00),NULL,0);
    //0x8D 0x14
    if (charge_pump == SSD130x_INTERNAL_PUMP)
    {
        val = SSD130x_CMD_CHARGE_INTERN;
        send_command(SSD130x_CMD_CHARGE_PUMP,&val,1);
    }

    //0x20 0x00
    ret = set_mem_addressing_mode(SSD130x_ADDR_TYPE_HORIZONTAL);
    if (ret != 0) {
        uBit->display.scroll("Mem Mode");
        return ret;
    }

    //0x21,0,127
    set_column_address(0,127);
    //return 0;
    //0x22,0,63
    set_page_address(0,7);
    //0xa0|0x1
    ret = set_read_direction();
    if (ret != 0) {
        uBit->display.scroll("Read Dir");
        return ret;
    }
    //0xC8
    ret = set_scan_direction();
    if (ret != 0) {
        uBit->display.scroll("Scan Dir");
        return ret;
    }

    //0xDA,0x12
    val = 0x12;
    send_command(SSD130x_CMD_COM_PIN_CONF,&val,1);
    set_contrast(0xFF);
    //0xd9 0xF1
    //0DB 0x40
    if (charge_pump == SSD130x_INTERNAL_PUMP)
    {
        val = 0xF1;
        send_command(SSD130x_CMD_SET_PRECHARGE, &val,1);
        val = SSD130x_VCOM_083;
        send_command(SSD130x_CMD_VCOM_LEVEL, &val,1);
    }
    send_command(SSD130x_CMD_PAGE_START_ADDR(0),NULL,0);

    ret = set_display_on(SSD130x_DISP_NORMAL);//A6
    return display_power(SSD130x_DISP_ON);
}

int ssd1306::power_off()
{
    return display_power(SSD130x_DISP_OFF);
}

int ssd1306::power_on()
{
    return display_power(SSD130x_DISP_ON);
}

int ssd1306::set_contrast(uint8_t ctrst)
{
    contrast = ctrst;
    return send_command(SSD130x_CMD_CONTRAST, &contrast,1);
}

int ssd1306::display_video_reverse()//FIXME params
{
    if (video_mode == SSD130x_DISP_REVERSE)
        return send_command(SSD130x_CMD_DISP_REVERSE, NULL, 0);
    else
        return send_command(SSD130x_CMD_DISP_NORMAL, NULL, 0);
}

#define CMD_BUF_SIZE 24
int ssd1306::send_command(uint8_t cmd, uint8_t* data, uint8_t len)
{
    char cmd_buf[CMD_BUF_SIZE] = {SSD130x_NEXT_BYTE_CMD,cmd};

    if (2*len > CMD_BUF_SIZE-2)
    {
        return -EINVAL;
    }

    int ret;
    if (len != 0)
    {
        for(int i = 0; i < len; i++)
        {
            cmd_buf[2+(2*i)] = SSD130x_NEXT_BYTE_CMD;
            cmd_buf[3+(2*i)] = data[i];
        }
    }

    ret = i2c->write(SSD130x_ADDR, cmd_buf, 2+(len*2));
    if( ret != MICROBIT_OK)
    {
        uBit->display.scroll("Command Error");
        return ret;
    }
    return 0;
}

int ssd1306::set_mem_addressing_mode(uint8_t mode)
{
    return send_command(SSD130x_CMD_ADDR_MODE,&mode,1);
}

int ssd1306::set_column_address(uint8_t col_start, uint8_t col_end)
{
    uint8_t buf[2] = {col_start,col_end};
    return send_command(SSD130x_CMD_COL_ADDR, buf, 2);
}

int ssd1306::set_page_address(uint8_t page_start, uint8_t page_end)
{
    uint8_t buf[2] = {page_start,page_end};
    return send_command(SSD130x_CMD_PAGE_ADDR, buf, 2);
}

int ssd1306::set_display_on(uint8_t use_ram)
{
    if (use_ram == SSD130x_DISP_BLANK)
    {
        return send_command(SSD130x_CMD_DISP_NORMAL, NULL,0);
    }
    else
    {
        return send_command(SSD130x_CMD_DISP_RAM, NULL,0);
    }
}

int ssd1306::display_power(uint8_t status)
{
    if (status == SSD130x_DISP_OFF)
    {
        return send_command(SSD130x_CMD_DISP_OFF, NULL,0);
    }
    else
    {
        return send_command(SSD130x_CMD_DISP_ON, NULL,0);
    }
}

int ssd1306::set_scan_direction() //FIXME
{
    if (scan_dir == SSD130x_SCAN_TOP_BOTTOM)
    {
        return send_command(SSD130x_CMD_COM_SCAN_NORMAL, NULL,0);
    }
    else
    {
        return send_command(SSD130x_CMD_COM_SCAN_REVERSE, NULL,0);
    }
}

int ssd1306::set_read_direction()
{
    if(read_dir == SSD130x_RIGHT_TO_LEFT)
        return send_command(SSD130x_CMD_SEG0_MAP_RIGHT, NULL,0);
    else
        return send_command(SSD130x_CMD_SEG0_MAP_LEFT, NULL,0);
}

int ssd1306::set_mux_ratio(uint8_t ratio)
{
    uint8_t data = SSD130x_MUX_DATA(ratio);
    return send_command(SSD130x_CMD_PAGE_ADDR, &data, 1);
}

int ssd1306::set_display_clock(uint8_t divide, uint8_t frequency)
{
    uint8_t data = SSD130x_CLK_DIV(divide) | SSD130x_CLK_FREQ(frequency);
    if (data != 0xF0)
        uBit->display.scroll("Error Clock");
    return send_command(SSD130x_CMD_DISP_CLK_DIV, &data, 1);
}

int ssd1306::set_display_offset(uint8_t dir, uint8_t nb_lines)
{
    uint8_t offset = 0;
    if (nb_lines >= SSD130x_NB_LINES)
    {
        return -EINVAL;
    }
    if (dir == SSD130x_MOVE_TOP)
        offset = SSD130x_OFFSET_DATA(nb_lines);
    else
        offset = SSD130x_OFFSET_DATA(SSD130x_NB_LINES - nb_lines);
    return send_command(SSD130x_CMD_DISPLAY_OFFSET, &offset, 1);
}

int ssd1306::update_screen()
{
    int ret;
    if (!fullscreen) {
        ret = set_column_address(0, 127);
        if (ret != 0)
            return ret;

        ret = set_page_address(0, 7);
        if (ret != 0)
            return ret;

        fullscreen = 1;
    }



    /* Setup I2C transfer */
    gddram[0] = SSD130x_DATA_ONLY;

    /* Send data on I2C bus */
    ret = i2c->write(SSD130x_ADDR, (char*) gddram,GDDRAM_SIZE+1);
    if (ret != MICROBIT_OK)
    {
        uBit->display.scroll("Full Screen Error");
    }
    return ret;
}

void ssd1306::display_char(uint8_t line, uint8_t col, uint8_t c)
{
    uint8_t tile = (c > FIRST_FONT_CHAR) ? (c - FIRST_FONT_CHAR) : 0;
    uint8_t* tile_data = (uint8_t*)(&font[tile]);
    buffer_set_tile(gddram, col, line, tile_data);
}

int ssd1306::display_line(uint8_t line, uint8_t col, const char* text)
{
    int len = strlen((char*)text);
    int i = 0;

    for (i = 0; i < len; i++) {
        uint8_t tile = (text[i] > FIRST_FONT_CHAR) ? (text[i] - FIRST_FONT_CHAR) : 0;
        uint8_t* tile_data = (uint8_t*)(&font[tile]);
        buffer_set_tile(gddram, col++, line, tile_data);
        if (col >= (OLED_LINE_CHAR_LENGTH)) {
            col = 0;
            line++;
            if (line >= SSD130x_NB_PAGES) {
                return i;
            }
        }
    }
    return len;
}

/* Set whole display to given value */
int ssd1306::buffer_set(uint8_t *gddram, uint8_t val)
{
    memset(gddram + 1, val, GDDRAM_SIZE);
    return 0;
}

/* Change our internal buffer, without actually displaying the changes */
int ssd1306::buffer_set_pixel(uint8_t* gddram, uint8_t x0, uint8_t y0, uint8_t state)
{
    uint8_t* addr = gddram + 1 + ((y0 / 8) * 128) + x0;
    if (state != 0) {
        *addr |=  (0x01 << (y0 % 8));
    } else {
        *addr &= ~(0x01 << (y0 % 8));
    }
    return 0;
}

/* Change a "tile" in the bitmap memory.
 * A tile is a 8x8 pixels region, aligned on a 8x8 grid representation of the display.
 *  x0 and y0 are in number of tiles.
 */
int ssd1306::buffer_set_tile(uint8_t* gddram, uint8_t x0, uint8_t y0, uint8_t* tile)
{
    uint8_t* addr = gddram + 1 + (y0 * 128) + (x0 * 8);
    memcpy(addr, tile, 8);
    return 0;
}
