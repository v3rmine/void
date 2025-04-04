/****************************************************************************
 *   extdrv/tsl256x_light_sensor.h
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

/**
 * This code is mainly an adaptation of code provided by Techno-Innov (Nathael Pajani <nathael.pajani@ed3l.fr>)
 * www.techno-innov.fr
 * http://git.techno-innov.fr
 */

#ifndef TSL256X_H
#define TSL256X_H

#include <cstdint>
#include "MicroBit.h"

#define TSL256x_ADDR 0x52


enum tsl256x_pkg_types {
    TSL256x_PACKAGE_T = 0,
    TSL256x_PACKAGE_FN,
    TSL256x_PACKAGE_CL,
    TSL256x_PACKAGE_CS,
};

struct tsl256x_internal_regs {
    uint8_t control; /* Control of basic functions */
    uint8_t timing;  /* Integration time/gain control */
    uint16_t low_int_threshold; /* low interrupt threshold, in little endian byte order */
    uint16_t high_int_threshold; /* high interrupt threshold, in little endian byte order */
    uint8_t interrupt; /* interrupt control */
    uint8_t reserved0[3];
    uint8_t part_id; /* Part number and revision ID */
    uint8_t reserved1;
    uint16_t data[2]; /* Data from both ADC, in little endian byte order */
};
#define TSL256x_REGS(x)   ((uint8_t)offsetof(struct tsl256x_internal_regs, x))

/* Defines for command byte */
#define TSL256x_CMD_REG_SELECT    (1 << 7)
#define TSL256x_INT_CLEAR         (1 << 6)
#define TSL256x_USE_WORD          (1 << 5)
#define TSL256x_USE_BLOCK         (1 << 4)
#define TSL256x_REG_ADDR(x)       ((x) & 0x0F)

#define TSL256x_CMD(x)   (TSL256x_CMD_REG_SELECT | TSL256x_REGS(x))

/* Defines for control register */
#define TSL256x_POWER_ON          (0x03)

/* Defines for timing register */
/* See page 22 of tsl256x manual for information on how to calculate lux. */
#define TSL256x_LOW_GAIN          (0x00)
#define TSL256x_HIGH_GAIN_16X     (1 << 4)
#define TSL256x_CONVERSION_START  (1 << 3)
#define TSL256x_CONVERSION_MANUAL (0x03)
#define TSL256x_INTEGRATION_400ms (0x02)
#define TSL256x_INTEGRATION_100ms (0x01)
#define TSL256x_INTEGRATION_13ms  (0x00)

/* Defines for interrupt control register */
#define TSL256x_INTR_NONE         (0x00)
#define TSL256x_INTR_LEVEL        (0x01 << 4)
#define TSL256x_INTR_SMBUS        (0x02 << 4)
#define TSL256x_INTR_ON_CONV_DONE (0x00)
#define TSL256x_INTR_NB_CYCLE(x)  ((x) & 0x0F)

/* Defines for part ID and revision ID register */
#define TSL256x_PART_ID(x)   (((x) & 0xF0) >> 4)
#define TSL256x_PART_REV(x)  ((x) & 0x0F)



/***************************************************************************** */
/* Lux Computation
 *
 * Copyright E 2004−2005 TAOS, Inc.
 *
 * THIS CODE AND INFORMATION IS PROVIDED ”AS IS” WITHOUT WARRANTY OF ANY KIND,
 * EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
 */

#define LUX_SCALE 14   /* scale by 2^14 */
#define RATIO_SCALE 9  /* scale ratio by 2^9 */

/* Integration time scaling factors */
#define CH_SCALE 10 /* scale channel values by 2^10 */
#define CHSCALE_TINT0 ((322 / 11) * (2 << CH_SCALE)) /* = 0x7517 = 322/11 * 2^CH_SCALE */
#define CHSCALE_TINT1 ((322 / 81) * (2 << CH_SCALE)) /* = 0x0fe7 = 322/81 * 2^CH_SCALE */

/*
 * T, FN, and CL Package coefficients
 *
 * For Ch1/Ch0=0.00 to 0.50 : Lux/Ch0=0.0304−0.062*((Ch1/Ch0)^1.4)
 *   piecewise approximation
 *     For Ch1/Ch0=0.00 to 0.125: Lux/Ch0=0.0304−0.0272*(Ch1/Ch0)
 *     For Ch1/Ch0=0.125 to 0.250: Lux/Ch0=0.0325−0.0440*(Ch1/Ch0)
 *     For Ch1/Ch0=0.250 to 0.375: Lux/Ch0=0.0351−0.0544*(Ch1/Ch0)
 *     For Ch1/Ch0=0.375 to 0.50: Lux/Ch0=0.0381−0.0624*(Ch1/Ch0)
 *
 * For Ch1/Ch0=0.50 to 0.61: Lux/Ch0=0.0224−0.031*(Ch1/Ch0)
 *
 * For Ch1/Ch0=0.61 to 0.80: Lux/Ch0=0.0128−0.0153*(Ch1/Ch0)
 *
 * For Ch1/Ch0=0.80 to 1.30: Lux/Ch0=0.00146−0.00112*(Ch1/Ch0)
 *
 * For Ch1/Ch0>1.3: Lux/Ch0=0
 *
 */
#define K1T 0x0040  /* 0.125 * 2^RATIO_SCALE */
#define B1T 0x01f2  /* 0.0304 * 2^LUX_SCALE */
#define M1T 0x01be  /* 0.0272 * 2^LUX_SCALE */
#define K2T 0x0080  /* 0.250 * 2^RATIO_SCALE */
#define B2T 0x0214 /* 0.0325 * 2^LUX_SCALE */
#define M2T 0x02d1 /* 0.0440 * 2^LUX_SCALE */
#define K3T 0x00c0 /* 0.375 * 2^RATIO_SCALE */
#define B3T 0x023f /* 0.0351 * 2^LUX_SCALE */
#define M3T 0x037b /* 0.0544 * 2^LUX_SCALE */
#define K4T 0x0100 /* 0.50 * 2^RATIO_SCALE */
#define B4T 0x0270 /* 0.0381 * 2^LUX_SCALE */
#define M4T 0x03fe /* 0.0624 * 2^LUX_SCALE */
#define K5T 0x0138 /* 0.61 * 2^RATIO_SCALE */
#define B5T 0x016f /* 0.0224 * 2^LUX_SCALE */
#define M5T 0x01fc /* 0.0310 * 2^LUX_SCALE */
#define K6T 0x019a /* 0.80 * 2^RATIO_SCALE */
#define B6T 0x00d2 /* 0.0128 * 2^LUX_SCALE */
#define M6T 0x00fb /* 0.0153 * 2^LUX_SCALE */
#define K7T 0x029a /* 1.3 * 2^RATIO_SCALE */
#define B7T 0x0018 /* 0.00146 * 2^LUX_SCALE */
#define M7T 0x0012 /* 0.00112 * 2^LUX_SCALE */
#define K8T 0x029a /* 1.3 * 2^RATIO_SCALE */
#define B8T 0x0000 /* 0.000 * 2^LUX_SCALE */
#define M8T 0x0000 /* 0.000 * 2^LUX_SCALE*/


/*
 * CS package coefficients
 *
 * For 0 <= Ch1/Ch0 <= 0.52 : Lux/Ch0 = 0.0315−0.0593*((Ch1/Ch0)^1.4)
 *   piecewise approximation
 *     For 0 <= Ch1/Ch0 <= 0.13 : Lux/Ch0 = 0.0315−0.0262*(Ch1/Ch0)
 *     For 0.13 <= Ch1/Ch0 <= 0.26 : Lux/Ch0 = 0.0337−0.0430*(Ch1/Ch0)
 *     For 0.26 <= Ch1/Ch0 <= 0.39 : Lux/Ch0 = 0.0363−0.0529*(Ch1/Ch0)
 *     For 0.39 <= Ch1/Ch0 <= 0.52 : Lux/Ch0 = 0.0392−0.0605*(Ch1/Ch0)
 *
 * For 0.52 < Ch1/Ch0 <= 0.65 : Lux/Ch0 = 0.0229−0.0291*(Ch1/Ch0)
 *
 * For 0.65 < Ch1/Ch0 <= 0.80 : Lux/Ch0 = 0.00157−0.00180*(Ch1/Ch0)
 *
 * For 0.80 < Ch1/Ch0 <= 1.30 : Lux/Ch0 = 0.00338−0.00260*(Ch1/Ch0)
 *
 * For Ch1/Ch0 > 1.30 : Lux = 0
 *
 */
#define K1C 0x0043 /* 0.130 * 2^RATIO_SCALE */
#define B1C 0x0204 /* 0.0315 * 2^LUX_SCALE */
#define M1C 0x01ad /* 0.0262 * 2^LUX_SCALE */
#define K2C 0x0085 /* 0.260 * 2^RATIO_SCALE */
#define B2C 0x0228 /* 0.0337 * 2^LUX_SCALE */
#define M2C 0x02c1 /* 0.0430 * 2^LUX_SCALE */
#define K3C 0x00c8 /* 0.390 * 2^RATIO_SCALE */
#define B3C 0x0253 /* 0.0363 * 2^LUX_SCALE */
#define M3C 0x0363 /* 0.0529 * 2^LUX_SCALE*/
#define K4C 0x010a /* 0.520 * 2^RATIO_SCALE */
#define B4C 0x0282 /* 0.0392 * 2^LUX_SCALE */
#define M4C 0x03df /* 0.0605 * 2^LUX_SCALE */
#define K5C 0x014d /* 0.65 * 2^RATIO_SCALE */
#define B5C 0x0177 /* 0.0229 * 2^LUX_SCALE */
#define M5C 0x01dd /* 0.0291 * 2^LUX_SCALE */
#define K6C 0x019a /* 0.80 * 2^RATIO_SCALE */
#define B6C 0x0101 /* 0.0157 * 2^LUX_SCALE */
#define M6C 0x0127 /* 0.0180 * 2^LUX_SCALE */
#define K7C 0x029a /* 1.3 * 2^RATIO_SCALE */
#define B7C 0x0037 /* 0.00338 * 2^LUX_SCALE */
#define M7C 0x002b /* 0.00260 * 2^LUX_SCALE */
#define K8C 0x029a /* 1.3 * 2^RATIO_SCALE */
#define B8C 0x0000 /* 0.000 * 2^LUX_SCALE */
#define M8C 0x0000 /* 0.000 * 2^LUX_SCALE*/



class tsl256x {

    public:
        /* Sensor config
         * Performs default configuration of the luminosity sensor.
         * FIXME : Add more comments about the behavior and the resulting configuration.
         */
        tsl256x(MicroBit* uB, MicroBitI2C* i2c, uint8_t addr = TSL256x_ADDR, uint8_t pkg = TSL256x_PACKAGE_T,
                uint8_t p_gain = TSL256x_LOW_GAIN, uint8_t integration = TSL256x_INTEGRATION_100ms);

        /* Check the sensor presence, return 1 if found
         * This is done by writing to the control register to set the power state to ON and
         *   reading the register to check the value, as stated in the TSL256x documentation page 14,
         *   (Register Set definitions)
         * FIXME : Never managed to read the required value, though the sensor is running and
         *     provides seemingly accurate values.
         */
        int probe_sensor();


        /* FIXME: add comments, and make it work right ... never managed to read the ID given in
         *   the documentation
         */
        int read_id();


        /* Sensor read
         * Performs a non-blocking read of the luminosity from the sensor.
         * 'lux' 'ir' and 'comb': integer addresses for conversion result, may be NULL.
         * Return value(s):
         *   Upon successfull completion, returns 0 and the luminosity read is placed in the
         *   provided integer(s). On error, returns a negative integer equivalent to errors from
         *   glibc.
         */
        int sensor_read(uint16_t* comb, uint16_t* ir, uint32_t* lux);



    private:
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
        uint32_t calculate_lux(uint16_t ch0, uint16_t ch1);

        MicroBit* uBit;
        MicroBitI2C* i2c;
        uint8_t address;
        uint8_t package;
        uint8_t gain;
        uint8_t integration_time;
        uint8_t probe_ok;

};


#endif /* TSL256X_H */


