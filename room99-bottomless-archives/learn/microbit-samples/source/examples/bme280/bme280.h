/****************************************************************************
 *   bme280.h
 *
 * BME280 I2C Barometric, humidity and temperature sensor driver
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

#ifndef BME280_H
#define BME280_H

#include <cstdint>
#include <MicroBit.h>

#define BME280_ADDR 0xEC

struct bme280_calibration_data {
    /* Temperature */
    uint16_t T1;   /* 0x88 */
    int16_t T2;    /* 0x8A */
    int16_t T3;    /* 0x8C */
    /* Pressure */
    uint16_t P1;   /* 0x8E */
    int16_t P2;    /* 0x90 */
    int16_t P3;    /* 0x92 */
    int16_t P4;    /* 0x94 */
    int16_t P5;    /* 0x96 */
    int16_t P6;    /* 0x98 */
    int16_t P7;    /* 0x9A */
    int16_t P8;    /* 0x9C */
    int16_t P9;    /* 0x9E */
    /* Humidity */
    uint8_t H1;    /* 0xA1 */
    int16_t H2;    /* 0xE1 .. 0xE2 */
    uint8_t H3;    /* 0xE3 */
    int16_t H4;    /* 0xE4 .. 0xE5[3:0] */
    int16_t H5;    /* 0xE5[7:4] .. 0xE6 */
    int8_t H6;     /* 0xE7 */
};

#define BME280_DATA_SIZE   8
struct bme280_data {
    uint8_t pressure[3];      /* 0xF7 .. 0xF9 */
    uint8_t temperature[3];  /* 0xFA .. 0xFC */
    uint8_t humidity[2];     /* 0xFD .. 0xFE */
} __attribute__ ((__packed__));


struct bme280_calibration_regs {
    /* Temperature */
    uint16_t T[3];   /* 0x88 .. 0x8D */
    /* Pressure */
    uint16_t P[9];   /* 0x8E .. 0x9F */
    /* Humidity */
    uint8_t res0[1];
    uint8_t Ha;    /* 0xA1 */
    uint8_t res1[63];
    uint8_t Hb[7];    /* 0xE1 .. 0xE7 */
};
#define BME280_CAL_REGS(x)   (((uint8_t)offsetof(struct bme280_calibration_regs, x)) + 0x88)
#define BME280_CAL_REGS_T_LEN   6
#define BME280_CAL_REGS_P_LEN   18
#define BME280_CAL_REGS_Ha_LEN  1
#define BME280_CAL_REGS_Hb_LEN  7
#define BME280_CAL_REGS_H_LEN   (BME280_CAL_REGS_Ha_LEN + BME280_CAL_REGS_Hb_LEN)

struct bme280_internal_regs {
    uint8_t chip_id;         /* 0xD0 - Value should be 0x60 */
    uint8_t reserved0[15];
    uint8_t reset;           /* 0xE0 */
    uint8_t reserved1[17];
    uint8_t ctrl_humidity;   /* 0xF2 */
    uint8_t status;          /* 0xF3 */
    uint8_t ctrl_measure;    /* 0xF4 */
    uint8_t config;          /* 0xF5 */
    uint8_t reserved2[1];
    union {
        uint8_t raw_data[8];   /* 0xF7 .. 0xFE */
        struct bme280_data data;
    };
} __attribute__ ((__packed__));
#define BME280_REGS(x)   (((uint8_t)offsetof(struct bme280_internal_regs, x)) + 0xD0)


#define BME280_ID   0x60
#define BME280_RESET_MAGIC  0xB6

/* Oversampling values */
#define BME280_SKIP     0x00
#define BME280_OS_x1    0x01
#define BME280_OS_x2    0x02
#define BME280_OS_x4    0x03
#define BME280_OS_x8    0x04
#define BME280_OS_x16   0x05

/* Mode values */
#define BME280_SLEEP    0x00
#define BME280_FORCED   0x01
#define BME280_NORMAL   0x03

/* Control registers helpers */
#define BME280_CTRL_HUM(hum)    ((hum) & 0x07)
#define BME280_CTRL_MEA(pres, temp, mode)  \
    ( (((temp) & 0x07) << 5) | (((pres) & 0x07) << 2) | ((mode) & 0x03) )


/* Standby */
#define BME280_SB_05ms     0x00
#define BME280_SB_10ms     0x06
#define BME280_SB_20ms     0x07
#define BME280_SB_62ms     0x01
#define BME280_SB_125ms    0x02
#define BME280_SB_250ms    0x03
#define BME280_SB_500ms    0x04
#define BME280_SB_1000ms   0x05

/* Filter */
#define BME280_FILT_OFF    0x00
#define BME280_FILT_2      0x01
#define BME280_FILT_4      0x02
#define BME280_FILT_8      0x03
#define BME280_FILT_16     0x04

/* Config register helper */
#define BME280_CONFIG(stb, filt)   ( (((stb) & 0x07) << 5) | (((filt) & 0x07) << 2) )


/* Data helpers */
#define BME280_DATA_20(msb, lsb, xlsb)  ((((msb) & 0xFF) << 12) | (((lsb) & 0xFF) << 4) | (((xlsb) & 0xF0) >> 4))
#define BME280_DATA_16(msb, lsb)        ((((msb) & 0xFF) << 8) | ((lsb) & 0xFF))

class bme280 {

    public:
        /* Sensor config
         * Performs configuration of the sensor and recovers calibration data from sensor's internal
         *   memory.
         */
        bme280(MicroBit* uB, MicroBitI2C* uBi2c, uint8_t addr = BME280_ADDR, uint8_t humidity_oversampling = BME280_OS_x16,
                uint8_t temp_oversampling = BME280_OS_x16, uint8_t pressure_oversampling = BME280_OS_x16, uint8_t mode = BME280_NORMAL,
                uint8_t standby_len = BME280_SB_62ms, uint8_t filter_coeff = BME280_FILT_OFF);

        /* Check the sensor presence, return 1 if found */
        int probe_sensor();

        /* Humidity, Temperature and Pressure Read
         * Performs a read of the data from the sensor.
         * 'hum', 'temp' and 'pressure': integer addresses for conversion result.
         * Return value(s):
         *   Upon successfull completion, returns 0 and the raw sensor values read are placed in the
         *   provided integer(s). On error, returns a negative integer equivalent to errors from
         *   glibc.
         */
        int sensor_read(uint32_t* pressure, int32_t* temp, uint16_t* hum);


        /* Compute actual temperature from uncompensated temperature
         * Param :
         *  - conf : bme280_sensor_configuration structure, with calibration data read from sensor
         *  - utemp : uncompensated (raw) temperature value read from sensor
         * Returns the value in 0.01 degree Centigrade
         * Output value of "5123" equals 51.23 DegC.
         */
        int compensate_temperature(int utemp);


        /* Compute actual pressure from uncompensated pressure
         * Returns the value in Pascal(Pa) or 0 on error (invalid value which would cause division by 0).
         * Output value of "96386" equals 96386 Pa = 963.86 hPa = 963.86 millibar
         */
        uint32_t compensate_pressure(int uncomp_pressure);


        /* Compute actual humidity from uncompensated humidity
         * Returns the value in 0.01 %rH
         * Output value of "4132" equals 41.32 %rH.
         */
        uint32_t compensate_humidity(int uncomp_humidity);

    private:

        int get_calibration_data();


        MicroBit* uBit;
        MicroBitI2C* i2c;
        uint8_t address;
        uint8_t probe_ok;
        uint8_t humidity_oversampling;
        uint8_t temp_oversampling;
        uint8_t pressure_oversampling;
        uint8_t mode;
        uint8_t standby_len;
        uint8_t filter_coeff;
        struct bme280_calibration_data cal;
        int fine_temp;

};

#endif /* BME280_H */


