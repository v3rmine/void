/****************************************************************************
 *   veml6070.h
 *
 * VEML6070 I2C UV sensor driver
 *
 * Copyright 2016 Nathael Pajani <nathael.pajani@ed3l.fr>
 * Copyright 2022 Anthony Chomienne <anthony@mob-dev.fr>
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

#ifndef VEML6070_H
#define VEML6070_H

#include <cstdint>
#include "MicroBit.h"




#define VEML6070_ADDR 0x70

/* Defines for command byte */
#define VEML6070_ACK               (1 << 5)
#define VEML6070_ACK_THD_102       (0)
#define VEML6070_ACK_THD_145       (1 << 4)
#define VEML6070_NO_ACK            (0)
#define VEML6070_INTEG_05T         (0x00 << 2)
#define VEML6070_INTEG_1T          (0x01 << 2)
#define VEML6070_INTEG_2T          (0x02 << 2)
#define VEML6070_INTEG_4T          (0x03 << 2)
#define VEML6070_DISABLE           (1 << 0)
#define VEML6070_ENABLE            (0)

class veml6070 {

    public:
        /* Sensor config
         * Performs default configuration of the UV sensor.
         */
        veml6070(MicroBit* uB, MicroBitI2C* i2c, uint8_t address = VEML6070_ADDR);

        /* Check the sensor presence, return 1 if found */
        int probe_sensor();


        /* UV Read
         * Performs a read of the uv data from the sensor.
         * 'uv_raw': integer addresses for conversion result.
         * Return value(s):
         *   Upon successfull completion, returns 0 and the value read is placed in the provided integer.
         *   On error, returns a negative integer equivalent to errors from glibc.
         */
        int sensor_read(uint16_t* uv_raw);



    private:
        MicroBit* uBit;
        MicroBitI2C* i2c;
        uint8_t address;
        uint8_t probe_ok;

};
#endif /* VEML6070_H */

