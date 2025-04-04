/****************************************************************************
 *   extdrv/bme280_humidity_sensor.c
 *
 * BME280 I2C Barometric, humidity and temperature sensor driver
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
#include <errno.h>

#include "bme280.h"


/* Sensor config
 * Performs configuration of the sensor and recovers calibration data from sensor's internal
 *   memory.
 * Return value:
 *   Upon successfull completion, returns 0. On error, returns a negative integer
 *   equivalent to errors from glibc.
 */
#define CONF_BUF_SIZE 6
bme280::bme280(MicroBit* uB, MicroBitI2C* uBi2c, uint8_t addr, uint8_t humidity_os, uint8_t temp_os,
                uint8_t pressure_os, uint8_t sensor_mode, uint8_t standby, uint8_t coeff)
                :
                uBit(uB), i2c(uBi2c), address(addr), probe_ok(0), humidity_oversampling(humidity_os), temp_oversampling(temp_os), pressure_oversampling(pressure_os), mode(sensor_mode), standby_len(standby), filter_coeff(coeff) 
{
    int ret = 0;
    char cmd_buf[CONF_BUF_SIZE] = {
        BME280_REGS(ctrl_humidity), (uint8_t)BME280_CTRL_HUM(humidity_oversampling),
        BME280_REGS(ctrl_measure),  (uint8_t)BME280_CTRL_MEA(pressure_oversampling, temp_oversampling, mode),
        BME280_REGS(config), (uint8_t)BME280_CONFIG(standby_len, filter_coeff),
    };

    if (probe_sensor() != 1) {
        probe_ok = 0;
        uBit->display.scroll("No Device");
    }
    /* Send the configuration */
    ret = i2c->write(address, cmd_buf, CONF_BUF_SIZE);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        uBit->display.scroll("Conf Error");
    }
    /* Get the calibration data */
    if (get_calibration_data() != MICROBIT_OK)
      uBit->display.scroll("Calibration Error");
}


/* Check the sensor presence, return 1 if found */
#define PROBE_BUF_SIZE  1
int bme280::probe_sensor()
{
    char cmd_buf[PROBE_BUF_SIZE] = {BME280_REGS(chip_id)};
    uint8_t id = 0;

    /* Did we already probe the sensor ? */
    if (probe_ok != 1) {
        i2c->write(address, cmd_buf,1,true);
        int ret = i2c->read(address, (char*)&id, 1);
        if (ret == MICROBIT_OK && id != BME280_ID) {
            probe_ok = 0;
        } else {
          probe_ok = 1;
        }
    }
    return probe_ok;
}


/* Get calibration data from internal sensor memory
 * These values are required to compute the pressure, temperature and humidity values
 *   from the uncompensated "raw" values read from the sensor ADC result registers.
 * Calibration data for pressure and temperature is aligned, packed, and in little
 *    endian byte order, so it is stored directly to the cal structure.
 * Calibration data for humidity is split among different registers and not aligned,
 *    so we use a temporary data buffer.
 * Note : On the LPC822 the I2C bus requires some time to go idle, so we need to
 *    sleep some time between two conscutive I2C access.
 *    this should be done in the I2C driver, but is not yet implemented, so it is
 *    done here. These msleep() calls may be removed for other micro-controllers, and
 *    when the I2C driver for the LPC822 is imporved.
 * Return value:
 *   Upon successfull completion, returns 0. On error, returns a negative integer
 *   equivalent to errors from glibc.
 */
#define CAL_CMD_SIZE  1
int bme280::get_calibration_data()
{
    int ret = 0;
    char cmd_buf[CAL_CMD_SIZE] = { BME280_CAL_REGS(T)};

    /* Give some time for I2C bus to go idle */
    uBit->sleep(1);
    /* Easy part : Temperature and Presure calibration data is packed, aligned, and
     *   in little endian byte order */
    /* Start by reading temperature calibration data */
    i2c->write(address,cmd_buf,1,true);
    ret = i2c->read(address, (char*)&(cal.T1), BME280_CAL_REGS_T_LEN);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    uBit->sleep(1); /* Again some time for I2C bus to go idle */
    /* Read pressure calibration data */
    cmd_buf[0] = BME280_CAL_REGS(P);
    i2c->write(address,cmd_buf,1,true);
    ret = i2c->read(address, (char*)&(cal.P1), BME280_CAL_REGS_P_LEN);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    uBit->sleep(1); /* ... */

    /* Read humidity calibration data. This one is split among bytes and not aligned. use
     *   temporary data buffer and then copy to calibration structure. */
    /* First part */
    uint8_t data[BME280_CAL_REGS_H_LEN];

    cmd_buf[0] = BME280_CAL_REGS(Ha);
    i2c->write(address,cmd_buf,1,true);
    ret = i2c->read(address, (char*)data, BME280_CAL_REGS_Ha_LEN);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    uBit->sleep(1); /* ... */
    /* Second part */
    cmd_buf[0] = BME280_CAL_REGS(Hb);
    i2c->write(address,cmd_buf,1,true);
    ret = i2c->read(address, (char*)data+BME280_CAL_REGS_Ha_LEN, BME280_CAL_REGS_Hb_LEN);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    /* And store in calibration structure */
    cal.H1 = data[0];
    cal.H2 = ((data[1] & 0xFF) | ((data[2] & 0xFF) << 8));
    cal.H3 = data[3];
    cal.H4 = (((data[4] & 0xFF) << 4) | (data[5] & 0x0F));
    cal.H5 = (((data[6] & 0xFF) << 4) | ((data[5] & 0xF0) >> 4));
    cal.H6 = data[7];

    return MICROBIT_OK;
}


/* Humidity, Temperature and Pressure Read
 * Performs a read of the data from the sensor.
 * 'hum', 'temp' and 'pressure': integer addresses for conversion result.
 * Return value(s):
 *   Upon successfull completion, returns 0 and the raw sensor values read are placed in the
 *   provided integer(s). On error, returns a negative integer equivalent to errors from
 *   glibc.
 */
#define READ_CMD_SIZE   1
int bme280::sensor_read(uint32_t* pressure, int32_t* temp, uint16_t* hum)
{
    int ret = 0;
    char cmd_buf[READ_CMD_SIZE] = { BME280_REGS(raw_data)};
    uint8_t data[BME280_DATA_SIZE];

    if (probe_ok != 1) {
        if (probe_sensor() != 1) {
            return -1;
        }
        uBit->sleep(1);
    }

    /* Start by reading all data */
    i2c->write(address,cmd_buf,1,true);
    ret = i2c->read(address, (char*)data, BME280_DATA_SIZE);
    if (ret != MICROBIT_OK) {
        probe_ok = 0;
        return ret;
    }
    if (pressure != NULL) {
        *pressure = BME280_DATA_20(data[0], data[1], data[2]);
    }
    if (temp != NULL) {
        *temp = BME280_DATA_20(data[3], data[4], data[5]);
    }
    if (hum != NULL) {
        *hum = BME280_DATA_16(data[6], data[7]);
    }

    return 0;
}



/* Compute actual temperature from uncompensated temperature
 * Param :
 *  - utemp : uncompensated (raw) temperature value read from sensor
 * Returns the value in 0.01 degree Centigrade
 * Output value of "5123" equals 51.23 DegC.
 */
int bme280::compensate_temperature(int utemp)
{
    int tmp1 = 0, tmp2 = 0;
    int temperature = 0;

    /* Calculate tmp1 */
    tmp1 = ((((utemp >> 3) - ((int)cal.T1 << 1))) * cal.T2) >> 11;
    /* Calculate tmp2 */
    tmp2 = (((utemp >> 4) - (int)cal.T1) * ((utemp >> 4) - (int)cal.T1)) >> 12;
    tmp2 = (tmp2 * cal.T3) >> 14;
    /* Calculate t_fine */
    fine_temp = tmp1 + tmp2;
    /* Calculate temperature */
    temperature = (fine_temp * 5 + 128) >> 8;
    return temperature;
}

/* Compute actual pressure from uncompensated pressure
 * Returns the value in Pascal(Pa) or 0 on error (invalid value which would cause division by 0).
 * Output value of "96386" equals 96386 Pa = 963.86 hPa = 963.86 millibar
 */
uint32_t bme280::compensate_pressure(int uncomp_pressure)
{
    int tmp1 = 0, tmp2 = 0, tmp3 = 0;
    uint32_t pressure = 0;

    /* Calculate tmp1 */
    tmp1 = (fine_temp >> 1) - 64000;
    /* Calculate tmp2 */
    tmp2 = (((tmp1 >> 2) * (tmp1 >> 2)) >> 11) * cal.P6;
    tmp2 = tmp2 + ((tmp1 * cal.P5) << 1);
    tmp2 = (tmp2 >> 2) + (cal.P4 << 16);
    /* Update tmp1 */
    tmp3 = (cal.P3 * (((tmp1 >> 2) * (tmp1 >> 2)) >> 13)) >> 3;
    tmp1 = (tmp3 + ((cal.P2 * tmp1) >> 1)) >> 18;
    tmp1 = (((32768 + tmp1)) * (int)cal.P1) >> 15;
    /* Calculate pressure */
    pressure = ((uint32_t)(1048576 - uncomp_pressure) - (tmp2 >> 12)) * 3125;

    /* Avoid exception caused by division by zero */
    if (tmp1 == 0) {
        return 0;
    }
    if (pressure < 0x80000000) {
        pressure = (pressure << 1) / ((uint32_t)tmp1);
    } else {
        pressure = (pressure / (uint32_t)tmp1) * 2;
    }

    tmp1 = (cal.P9 * ((int)(((pressure >> 3) * (pressure >> 3)) >> 13))) >> 12;
    tmp2 = (((int)(pressure >> 2)) * cal.P8) >> 13;
    pressure = (uint32_t)((int)pressure + ((tmp1 + tmp2 + cal.P7) >> 4));

    return pressure;
}


/* Compute actual humidity from uncompensated humidity
 * Returns the value in 0.01 %rH
 * Output value of "4132" equals 41.32 %rH.
 */
uint32_t bme280::compensate_humidity(int uncomp_humidity)
{
    int tmp1 = 0, tmp2 = 0, tmp3 = 0;
    uint32_t humidity = 0;

    /* Calculate tmp1 */
    tmp1 = fine_temp - 76800;
    /* Calculate tmp2 */
    tmp2 = ((uncomp_humidity << 14) - (cal.H4 << 20) - (cal.H5 * tmp1) + 16384) >> 15;
    /* Calculate tmp3 */
    tmp3 = ((((tmp1 * cal.H6) >> 10) * (((tmp1 * (int)cal.H3) >> 11) + 32768)) >> 10) + 2097152;
    /* Update tmp1 */
    tmp1 = tmp2 * ((tmp3 * cal.H2 + 8192) >> 14);
    tmp1 = tmp1 - (((((tmp1 >> 15) * (tmp1 >> 15)) >> 7) * (int)cal.H1) >> 4);
    if (tmp1 < 0) {
        tmp1 = 0;
    }
    if (tmp1 > 419430400) {
        tmp1 = 419430400;
    }
    humidity = (uint32_t)(tmp1 >> 12);
    /* Convert from 32bit integer in Q22.10 format (22 integer 10 fractional bits) to a value
     *   in 0.01 %rH :
     * A value of 42313 represents 42313 / 1024 = 41.321 %rH, convert it to 4132, which is 41.32 %rH.
     */
    humidity = ((humidity >> 10) * 100) + ((((humidity & 0x3FF) * 1000) >> 10) / 10);
    return humidity;

}
