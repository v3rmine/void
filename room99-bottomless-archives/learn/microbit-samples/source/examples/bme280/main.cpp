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
#include "bme280.h"

MicroBit uBit;
MicroBitI2C i2c(I2C_SDA0,I2C_SCL0);

int main()
{
    // Initialise the micro:bit runtime.
    uBit.init();

    bme280 bme(&uBit,&i2c);
    uint32_t pressure = 0;
    int32_t temp = 0;
    uint16_t humidite = 0;

    while(true)
    {
        bme.sensor_read(&pressure, &temp, &humidite);
        int tmp = bme.compensate_temperature(temp);
        int pres = bme.compensate_pressure(pressure)/100;
        int hum = bme.compensate_humidity(humidite);
        ManagedString display = "Temp:" + ManagedString(tmp/100) + "." + (tmp > 0 ? ManagedString(tmp%100): ManagedString((-tmp)%100))+" C";
        uBit.display.scroll(display.toCharArray());
        display = "Humi:" + ManagedString(hum/100) + "." + ManagedString(tmp%100)+" rH";
        uBit.display.scroll(display.toCharArray());
        display = "Pres:" + ManagedString(pres)+" hPa";
        uBit.display.scroll(display.toCharArray());
        uBit.sleep(1000);
    }

    release_fiber();

}

