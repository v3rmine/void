/****************************************************************************
 *   extdrv/veml6070_uv_sensor.c
 *
 * VEML6070 I2C UV sensor driver
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


#include "veml6070.h"


/* Sensor config
 * Performs default configuration of the UV sensor.
 */
#define CONF_BUF_SIZE 1
veml6070::veml6070(MicroBit* uB, MicroBitI2C* uBi2c, uint8_t addr): uBit(uB), i2c(uBi2c), address(addr)
{
    int ret = 0;
    char cmd_buf[CONF_BUF_SIZE] = { (VEML6070_NO_ACK | VEML6070_INTEG_1T | VEML6070_ENABLE), };

    if (probe_sensor() != 1) {
        uBit->display.scroll("VEML6070: No Device");
    }
	uBit->sleep(1);
    ret = i2c->write(address,cmd_buf, CONF_BUF_SIZE);
    if (ret != MICROBIT_OK) {
		probe_ok = 0;
        uBit->display.scroll("VEML6070: Conf Error");
    }
}



/* Check the sensor presence, return 1 if found */
int veml6070::probe_sensor()
{
	char dropped;

    /* Did we already probe the sensor ? */
    if (probe_ok != 1) {
        int status = i2c->read(address, &dropped, 1);
        if (status == MICROBIT_OK)
            probe_ok = 1;

    }
    return probe_ok;
}

/* UV Read
 * Performs a read of the uv data from the sensor.
 * 'uv_raw': integer addresses for conversion result.
 * Return value(s):
 *   Upon successfull completion, returns 0 and the luminosity read is placed in the
 *   provided integer(s). On error, returns a negative integer (-1 NO Device, -2 Invalid uv_raw input value, -3 read error)
 *
 */
int veml6070::sensor_read(uint16_t* uv_raw)
{
    int ret = 0;
    uint8_t data = 0;

    if (probe_ok != 1) {
	    if (probe_sensor() != 1) {
			uBit->display.scroll("VEML6070: No Device");
            return -1;
	    }
		uBit->sleep(1);
	}
    if (uv_raw == NULL) {
        return -2;
    }

	/* Start by reading MSB */
    ret = i2c->read(address+2, (char*)&data, 1);
    if (ret != MICROBIT_OK) {
		probe_ok = 0;
        return ret;
    }
    *uv_raw = ((uint16_t)data << 8) & 0xFF00;
    uBit->sleep(1);
	/* And read LSB */
    ret = i2c->read(address, (char*)&data, 1);
    if (ret != MICROBIT_OK) {
        return ret;
    }
    *uv_raw |= (uint16_t)data & 0x00FF;

    return 0;
}
