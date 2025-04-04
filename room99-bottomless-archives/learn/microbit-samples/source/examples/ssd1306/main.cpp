/*
   The MIT License (MIT)

   Copyright (c) 2016 British Broadcasting Corporation.
   This software is provided by Lancaster University by arrangement with the BBC.

   Permission is hereby granted, free of charge, to any person obtaining a
   copy of this software and associated documentation files (the "Software"),
   to deal in the Software without restriction, including without limitation
   the rights to use, copy, modify, merge, publish, distribute, sublicense,
   and/or sell copies of the Software, and to permit persons to whom the
   Software is furnished to do so, subject to the following conditions:

   The above copyright notice and this permission notice shall be included in
   all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
   THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
   FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
   DEALINGS IN THE SOFTWARE.
   */

#include "MicroBit.h"
#include "ssd1306.h"

MicroBit uBit;
MicroBitI2C i2c(I2C_SDA0,I2C_SCL0);
MicroBitPin P0(MICROBIT_ID_IO_P0, MICROBIT_PIN_P0, PIN_CAPABILITY_DIGITAL_OUT);

int main()
{
    // Initialise the micro:bit runtime.
    uBit.init();

    ssd1306 screen(&uBit, &i2c, &P0);
    while(true)
    {
        screen.display_line(0,0,"ABCDEFGHIJKLMNOP");
        screen.display_line(1,0,"BCDEFGHIJKLMNOP");
        screen.display_line(2,0,"CDEFGHIJKLMNOP");
        screen.display_line(3,0,"DEFGHIJKLMNOP");
        screen.display_line(4,0,"EFGHIJKLMNOP");
        screen.display_line(5,0,"FGHIJKLMNOP");
        screen.display_line(6,0,"GHIJKLMNOP");
        screen.display_line(7,0,"HIJKLMNOP");
        screen.update_screen();
        uBit.sleep(1000);
    }

    release_fiber();

}

