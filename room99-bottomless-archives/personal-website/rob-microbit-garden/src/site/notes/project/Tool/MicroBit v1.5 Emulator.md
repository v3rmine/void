---
{"dg-publish":true,"permalink":"/project/tool/micro-bit-v1-5-emulator/","tags":["Budding"]}
---


Topics: [[Personal Project\|Personal Project]]

- https://tech.microbit.org/hardware/1-5-revision/
- [BBC Technical Specifications](http://www.bbc.co.uk/mediacentre/mediapacks/microbit/specs)
- [I2C specification (behind login)](https://www.nxp.com/webapp/Download?colCode=UM10204&location=null)
- [SPI ‘specification’](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus)
- [Fritzing diagram, contributed by Kok Ho Huen](https://tech.microbit.org/docs/hardware/assets/Microbit.fzpz.zip)

# Peripherals
- Micro USB
	- MSC
	- UART
	- CMSIS-DAP
	- webUSB
- 5x5 LED Matrix
- 2 User buttons
- IO Pins
	- SPI
	- UART
	- I2C
- Extenal 3.3V supply
- 2.4GHz Antenna
	- Bluetooth low energy
	- Broadcast radio
- Battery connector (JST 3V connection)
- Nordic nRF51822 (CPU)
- Motion Sensor (ST LSM303AGR)
- NXP KL26Z (USB Interface chip)
- Reset Button

## nRF51
| item         | details                                                                                                                      |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| Model        | **[Nordic nRF51822-QFAA-R rev 3](https://www.nordicsemi.com/eng/Products/Bluetooth-low-energy/nRF51822)**                        |
| Core variant | **[Arm Cortex-M0 32 bit processor](https://www.arm.com/products/processors/cortex-m/cortex-m0.php)**                             |
| Flash ROM    | **256KB**                                                                                                                        |
| RAM          | **16KB**                                                                                                                         |
| Speed        | **16MHz**                                                                                                                        |
| Debug        | SWD, jlink/OB                                                                                                                |
| More Info    | **[Software](https://tech.microbit.org/software), [NRF51 datasheet](http://infocenter.nordicsemi.com/pdf/nRF51822_PS_v3.1.pdf)** |

## Bluetooth (via Nordic S110 SoftDevice)
| item                 | details                                                                                               |
| -------------------- | ----------------------------------------------------------------------------------------------------- |
| Stack                | Bluetooth 4.1 with Bluetooth low energy                                                               |
| Band                 | 2.4GHz ISM (Industrial, Scientific and Medical) 2.4GHz..2.41GHz                                       |
| Channels             | 50 2MHz channels, only 40 used (0 to 39), 3 advertising channels (37,38,39)                           |
| Sensitivity          | -93dBm in Bluetooth low energy mode                                                                   |
| Tx Power             | -20dBM to 4dBm in 4 dB steps                                                                          |
| Role                 | [GAP Peripheral](http://bluetooth-developer.blogspot.co.uk/2016/07/microbit-and-bluetooth-roles.html) |
| Congestion avoidance | Adaptive Frequency Hopping                                                                            |
| Profiles             | [BBC micro:bit profile](https://lancaster-university.github.io/microbit-docs/ble/profile/)            |
| More Info            | [Bluetooth](https://tech.microbit.org/bluetooth)                                                      |

## Low Level Radio (including  Nordic Gazell protocol => interface radio)
| item         | details                                                                                |
| ------------ | -------------------------------------------------------------------------------------- |
| Protocol     | **[Micro:bit Radio](https://lancaster-university.github.io/microbit-docs/ubit/radio)** |
| Freq band    | 2.4GHz                                                                                 |
| Channel rate | 1Mbps or 2Mbps                                                                         |
| Encryption   | None                                                                                   |
| Channels     | 101 (0..100)                                                                           |
| Group codes  | 255                                                                                    |
| Tx power     | Eight user configurable settings from 0(-30dbm) to 7 (+4dbm)                           |
| Payload size | 32 (standard) 255 (if reconfigured)                                                    |
| More Info    | [Micro:bit Radio](https://lancaster-university.github.io/microbit-docs/ubit/radio)     |

## Buttons
- A and B => connected to GPIO pins
- 0 logical when pressed

| item     | details                                         |
| -------- | ----------------------------------------------- |
| Type     | 2 tactile user buttons, 1 tactile system button |
| Debounce | (A & B) software debounced, 54ms period         |
| Pullup   | (A & B) external 4K7, (System) 10K              |

## 5x5 LED Matrix
- Can see ambient light level

| item                 | details                                         |
| -------------------- | ----------------------------------------------- |
| Type                 | miniature surface mount red LED                 |
| Physical structure   | 5x5 matrix                                      |
| Electrical structure | 3x9                                             |
| Intensity control    | 10 steps                                        |
| Sensing              | ambient light estimation via software algorithm |
| Sensing Range        | 10 levels from off to full on                   |
| Colour sensitivity   | red centric, red is 700nm                       |

## Motion sensor
| item              | details                                                               |
| ----------------- | --------------------------------------------------------------------- |
| Model             | [LSM303GR](https://www.st.com/en/mems-and-sensors/lsm303agr.html)     |
| Features          | 3 magnetic field and 3 acceleration axes , 2/4/8/16g ranges           |
| Resolution        | 8/10/12 bits                                                          |
| On board gestures | "freefall"                                                            |
| Other gestures    | Other gestures are implemented by software algorithms in the runtime. |

## Temperature Sensing
| item          | details                                                                                   |
| ------------- | ----------------------------------------------------------------------------------------- |
| Type          | on-core nRF51                                                                             |
| Sensing range | -25C .. 75C                                                                               |
| Resolution    | 0.25C steps                                                                               |
| Accuracy      | +/-4C (uncalibrated)                                                                      |
| More Info     | [DAL Thermometer](https://lancaster-university.github.io/microbit-docs/ubit/thermometer/) |

## GPIO
| item           | details                                                                            |
| -------------- | ---------------------------------------------------------------------------------- |
| Rings          | 3 large IO rings and two large power rings, 4mm plug and crocodile clip compatible |
| GPIO features  | 19 assignable GPIO pins                                                            |
|                | 2 are assigned to the on-board I2C interface                                       |
|                | 6 are used for display or light sensing feature                                    |
|                | 2 are used for on-board button detection                                           |
|                | 1 is reserved for an accessibility interface                                       |
|                | 19 may be assigned as digital input or digital output                              |
|                | 19 may be assigned for up to 3 simultaneous PWM channels                           |
|                | 19 may be assigned for 1 serial transmit and 1 serial receive channel              |
|                | 6 may be assigned as analog input pins                                             |
|                | 3 may be assigned to an optional SPI communications interface                      |
|                | 3 may be assigned for up to 3 simultaneous touch sensing inputs                    |
| ADC resolution | 10 bit (0..1023)                                                                   |
| Edge Connector | [Edge connector](https://tech.microbit.org/hardware/edgeconnector/)                |
| Pitch          | 1.27mm, 80 way double sided.                                                       |
| Pads           | 5 pads, with 4mm holes                                                             |

## Interface chip (USB)
| item               | details                                                                                                                                                                                                                                                         |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Model              | [Freescale MKL26Z128VFM4](http://www.nxp.com/webapp/search.partparamdetail.framework?PART_NUMBER=MKL26Z128VFM4)                                                                                                                                                 |
| Core variant       | [Arm Cortex-M0+](https://www.arm.com/products/processors/cortex-m/cortex-m0plus.php)                                                                                                                                                                            |
| Flash ROM          | 128KB                                                                                                                                                                                                                                                           |
| RAM                | 16KB                                                                                                                                                                                                                                                            |
| Speed              | [16Mhz (crystal)](https://github.com/Armmbed/DAPLink/blob/f499eb6ec4a847a2b78831fe1acc856fd8eb2f28/source/hic_hal/freescale/kl26z/MKL26Z4/system_MKL26Z4.c#L69) 48MHz (max)                                                                                     |
| Debug capabilities | SWD                                                                                                                                                                                                                                                             |
| More Info          | [DAPLink](https://tech.microbit.org/software/daplink-interface/), [KL26 reference manual (behind login)](https://www.nxp.com/webapp/Download?colCode=KL26P121M48SF4RM) [KL26Z datasheet](http://www.nxp.com/docs/pcn_attachments/16440_KL26P64M48SF5_Rev.4.pdf) |

## USB Communication
| item                  | details                                                                                            |
| --------------------- | -------------------------------------------------------------------------------------------------- |
| USB version           | 1.1 Full Speed device                                                                              |
| Speed                 | 12Mbit/sec                                                                                         |
| USB classes supported | [Mass Storage Class (MSC)](https://en.wikipedia.org/wiki/USB_mass_storage_device_class)            |
|                       | [Communications Device Class (CDC)](https://en.wikipedia.org/wiki/USB_communications_device_class) |
|                       | [CMSIS-DAP HID & WinUSB](https://arm-software.github.io/CMSIS_5/DAP/html/index.html)               |
|                       | [WebUSB CMSIS-DAP HID](https://wicg.github.io/webusb/)                                             |
| More Info             | [DAPLink](https://tech.microbit.org/software/daplink-interface/)                                   |

## Debugging
| item      | details                                                                                                         |
| --------- | --------------------------------------------------------------------------------------------------------------- |
| Protocol  | Serial Wire Debug (SWD)                                                                                         |
| Options   | DAPLink (CMSIS-DAP)                                                                                             |
|           | JLink/OB (via different firmware)                                                                               |
| More Info | [Mbed debugging micro:bit](https://docs.mbed.com/docs/mbed-os-handbook/en/latest/debugging/debugging_microbit/) |

# ARMv6-M Instructions
https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/The-ARMv6-M-Instruction-Set

| Instruction | Opcode | Small Desc |
| ----------- | ------ | ---------- |
|             |        |            |
