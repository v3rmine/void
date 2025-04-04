/****************************************************************************
 *   extdrv/tsl256x_light_sensor.c
 *
 * TSL256x I2C luminosity and IR sensor driver
 *
 * Copyright 2016 Nathael Pajani <nathael.pajani@ed3l.fr>
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


#include <cstdint>

#include "tsl256x.h"

/* Sensor config
 * Performs default configuration of the luminosity sensor.
 * FIXME : Add more comments about the behavior and the resulting configuration.
 */
#define CONF_BUF_SIZE 2
tsl256x::tsl256x(MicroBit* uB, MicroBitI2C* uBi2c, uint8_t addr, uint8_t pkg,
        uint8_t p_gain, uint8_t integration)
    :
        uBit(uB), i2c(uBi2c), address(addr), package(pkg), gain(p_gain), integration_time(integration)
{
    probe_ok = 0;
    int ret = 0;
    char cmd_buf[CONF_BUF_SIZE] = { TSL256x_CMD(timing), 0, };

    cmd_buf[1] = (gain | integration_time);

    if (probe_sensor() != 1) {
        uBit->display.scroll("TSL256X: No Device");
    }
    uBit->sleep(1);
    ret = i2c->write(address, cmd_buf, CONF_BUF_SIZE);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        uBit->display.scroll("TSL256x: Conf Error");
    }
}



/* Check the sensor presence, return 1 if found
 * This is done by writing to the control register to set the power state to ON and
 *   reading the register to check the value, as stated in the TSL256x documentation page 14,
 *   (Register Set definitions)
 * FIXME : Never managed to read the required value, though the sensor is running and
 *     provides seemingly accurate values.
 */
#define PROBE_BUF_SIZE  2
int tsl256x::probe_sensor()
{
    int ret = 0;
    char cmd_buf[PROBE_BUF_SIZE] = { TSL256x_CMD(control), TSL256x_POWER_ON, };
    uint8_t control_val = 72;

    /* Did we already probe the sensor ? */
    if (probe_ok != 1) {
        ret = i2c->write(address, cmd_buf, PROBE_BUF_SIZE);
        if (ret != MICROBIT_OK) {
            return 0;
        }
        uBit->sleep(500);
        i2c->write(address, cmd_buf,1,true);
        ret = i2c->read(address, (char*)&control_val, 1);
        /* FIXME : check that control_val is TSL256x_POWER_ON... */
        if (ret == MICROBIT_OK && (control_val&0x03) == TSL256x_POWER_ON) {
            probe_ok = 1;
        }
    }
    return probe_ok;
}



/* FIXME: add comments, and make it work right ... never managed to read the ID given in
 *   the documentation
 */

#define ID_BUF_SIZE  1
int tsl256x::read_id()
{
    int ret = 0;
    char cmd_buf[ID_BUF_SIZE] = { TSL256x_CMD(part_id)};
    char id = 0;

    /* Did we already probe the sensor ? */
    if (probe_ok != 1) {
        return 0;
    }
    i2c->write(address, cmd_buf, ID_BUF_SIZE, true);
    ret = i2c->read(address, &id, 1);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    return id;
}


/* Lux Read
 * Performs a non-blocking read of the luminosity from the sensor.
 * 'lux' 'ir' and 'comb': integer addresses for conversion result, may be NULL.
 * Return value(s):
 *   Upon successfull completion, returns 0 and the luminosity read is placed in the
 *   provided integer(s). On error, returns a negative integer equivalent to errors from
 *   glibc.
 */
#define READ_BUF_SIZE  1
int tsl256x::sensor_read(uint16_t* comb, uint16_t* ir, uint32_t* lux)
{
    int ret = 0;
    char cmd_buf[READ_BUF_SIZE] = { TSL256x_CMD(data)};
    uint8_t data[4];
    uint16_t comb_raw = 0, ir_raw = 0;

    i2c->write(address, cmd_buf, READ_BUF_SIZE,true);
    ret = i2c->read(address, (char*)data, 4);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    comb_raw = (data[0] & 0xFF) | ((data[1] << 8) & 0xFF00);
    ir_raw = (data[2] & 0xFF) | ((data[3] << 8) & 0xFF00);

    if (comb != NULL) {
        *comb = comb_raw;
    }
    if (ir != NULL) {
        *ir = ir_raw;
    }
    if (lux != NULL) {
        *lux = calculate_lux(comb_raw, ir_raw);
    }

    return 0;
}




/***************************************************************************** */
/*
 * lux equation approximation without floating point calculations
 *
 * Description:
 *   Calculate the approximate illuminance (lux) given the raw channel values of
 *   the TSL2560. The equation if implemented as a piece−wise linear approximation.
 *
 * Arguments:
 * uint16_t ch0 − raw channel value from channel 0 of TSL2560
 * uint16_t ch1 − raw channel value from channel 1 of TSL2560
 *
 * Return: uint32_t − the approximate illuminance (lux)
 *
 */
uint32_t tsl256x::calculate_lux(uint16_t ch0, uint16_t ch1)
{
    /* First, scale the channel values depending on the gain and integration time
     * 16X, 402mS is nominal.
     * Scale if integration time is NOT 402 msec */
    uint32_t chScale = 0;
    uint32_t channel1 = 0, channel0 = 0;
    uint32_t ratio = 0, lux = 0;
    uint32_t b = 0, m = 0;

    switch (integration_time) {
        case TSL256x_INTEGRATION_13ms: /* 13.7 msec */
            chScale = CHSCALE_TINT0;
            break;
        case TSL256x_INTEGRATION_100ms: /* 101 msec */
            chScale = CHSCALE_TINT1;
            break;
        case TSL256x_INTEGRATION_400ms: /* 402 msec */
        default: /* assume no scaling */
            chScale = (1 << CH_SCALE);
            break;
    }

    /* Scale if gain is NOT 16X */
    if (gain == TSL256x_LOW_GAIN) {
        chScale = chScale << 4; /* Scale 1X to 16X */
    }

    // Scale the channel values */
    channel0 = (ch0 * chScale) >> CH_SCALE;
    channel1 = (ch1 * chScale) >> CH_SCALE;

    /* Find the ratio of the channel values (Channel1/Channel0) */
    /* Protect against divide by zero */
    if (channel0 != 0) {
        ratio = (channel1 << (RATIO_SCALE + 1)) / channel0;
    }
    /* Round the ratio value */
    ratio = (ratio + 1) >> 1;

    /* Is ratio <= eachBreak ? */
    switch (package) {
        case TSL256x_PACKAGE_T:
        case TSL256x_PACKAGE_FN:
        case TSL256x_PACKAGE_CL:
            if (ratio <= K1T) {
                b = B1T; m = M1T;
            } else if (ratio <=  K2T) {
                b = B2T; m = M2T;
            } else if (ratio <=  K3T) {
                b = B3T; m = M3T;
            } else if (ratio <=  K4T) {
                b = B4T; m = M4T;
            } else if (ratio <=  K5T) {
                b = B5T; m = M5T;
            } else if (ratio <=  K6T) {
                b = B6T; m = M6T;
            } else if (ratio <=  K7T) {
                b = B7T; m = M7T;
            } else if (ratio > K8T) {
                b = B8T; m = M8T;
            } break;
        case TSL256x_PACKAGE_CS:    /* CS package */
            if (ratio <=  K1C) {
                b = B1C; m = M1C;
            } else if (ratio <=  K2C) {
                b = B2C; m = M2C;
            } else if (ratio <=  K3C) {
                b = B3C; m = M3C;
            } else if (ratio <=  K4C) {
                b = B4C; m = M4C;
            } else if (ratio <=  K5C) {
                b = B5C; m = M5C;
            } else if (ratio <=  K6C) {
                b = B6C; m = M6C;
            } else if (ratio <=  K7C) {
                b = B7C; m = M7C;
            }else if (ratio > K8C) {
                b = B8C; m = M8C;
            } break;
    }
    lux = ((channel0 * b) - (channel1 * m));

    /* Round lsb (2^(LUX_SCALE−1)) */
    lux += (1 << (LUX_SCALE - 1));
    /* Strip off fractional portion */
    lux = lux >> LUX_SCALE;

    return lux;
}

