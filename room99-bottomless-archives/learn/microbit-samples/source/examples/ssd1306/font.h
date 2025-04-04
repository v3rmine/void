/************************************************************************
 * font.h
 *
 * Copyright (C) Lisa Milne 2014 <lisa@ltmnet.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>
 *
 ************************************************************************/

/*
 * The font has been found on opengameart.org:
 * http://opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
 */

#ifndef FONT_H
#define FONT_H

#include <cstdint>

#define NB_FONT_TILES  95
#define FIRST_FONT_CHAR  0x20


#define NORMAL(x)   (x)

/*
 * Actual font data is written in "human drawing order", but must be sent in revers bit order
 *   for some drivers (Epaper for example).
 * This macro takes care of bit reversal for each font byte once and for all.
 * This is a little bit ugly, but made at compile time, which allows storage of modified font
 *   in FLASH memory.
 */
#define REV1(x)      ( (((x) >> 1) & 0x5555555555555555) | (((x) & 0x5555555555555555) << 1) )
#define REV2(x)      ( ((REV1(x) >> 2) & 0x3333333333333333) | ((REV1(x) & 0x3333333333333333) << 2) )
#define REVERSE(x)   ( ((REV2(x) >> 4) & 0x0F0F0F0F0F0F0F0F) | ((REV2(x) & 0x0F0F0F0F0F0F0F0F) << 4) )


/*
 * Actual font data is written in "human drawing order", but must be sent in a "vertical" way
 *   for some drivers (Oled for example).
 * This macro takes care of bit reversal for each font byte once and for all.
 * This is a little bit ugly, but made at compile time, which allows storage of modified font
 *   in FLASH memory.
 */
#define SBV(x, n)     ( ((x) & (0x01ULL << (n * 8))) >> (n * 7) )
#define VER1(x)      ( SBV(x, 7) | SBV(x, 6) | SBV(x, 5) | SBV(x, 4) | SBV(x, 3) | SBV(x, 2) | SBV(x, 1) | SBV(x, 0) )
#define VER2(x, n)   ( (VER1((x) >> n)) << (n * 8) )
#define VERTICAL(x)  ( VER2(x, 0) | VER2(x, 1) | VER2(x, 2)| VER2(x, 3)| VER2(x, 4)| VER2(x, 5)| VER2(x, 6)| VER2(x, 7) )


/*
 * Actual font data is written in "human drawing order", but must be sent in a "vertical" way
 *   for some drivers (Oled for example).
 * This macro takes care of bit reversal for each font byte once and for all.
 * This is a little bit ugly, but made at compile time, which allows storage of modified font
 *   in FLASH memory.
 */
#define SBR(x, n)   REVERSE( SBV(x, n) )
#define VR1(x)      ( SBR(x, 7) | SBR(x, 6) | SBR(x, 5) | SBR(x, 4) | SBR(x, 3) | SBR(x, 2) | SBR(x, 1) | SBR(x, 0) )
#define VR2(x, n)   ( (VR1((x) >> (7 - n))) << (n * 8) )
#define VERTICAL_REV(x)  ( VR2(x, 0) | VR2(x, 1) | VR2(x, 2)| VR2(x, 3)| VR2(x, 4)| VR2(x, 5)| VR2(x, 6)| VR2(x, 7) )


/*
 * The values in this array are a 8x8 bitmap font for ascii characters
 * As memory is a very precious ressource on a micro-controller all chars
 * before "space" (0x20) have been removed.
 */
#define FONT_TABLE \
	ROW(0x0000000000000000), /* (space) */  /* 0x20 */ \
	ROW(0x0808080800080000), /* ! */                   \
	ROW(0x2828000000000000), /* " */                   \
	ROW(0x00287C287C280000), /* # */                   \
	ROW(0x081E281C0A3C0800), /* $ */                   \
	ROW(0x6094681629060000), /* % */                   \
	ROW(0x1C20201926190000), /* & */                   \
	ROW(0x0808000000000000), /* ' */                   \
	ROW(0x0810202010080000), /* ( */                   \
	ROW(0x1008040408100000), /* ) */                   \
	ROW(0x2A1C3E1C2A000000), /* * */                   \
	ROW(0x0008083E08080000), /* + */                   \
	ROW(0x0000000000081000), /* , */                   \
	ROW(0x0000003C00000000), /* - */                   \
	ROW(0x0000000000080000), /* . */                   \
	ROW(0x0204081020400000), /* / */                   \
	ROW(0x1824424224180000), /* 0 */ /*x30 */          \
	ROW(0x08180808081C0000), /* 1 */                   \
	ROW(0x3C420418207E0000), /* 2 */                   \
	ROW(0x3C420418423C0000), /* 3 */                   \
	ROW(0x081828487C080000), /* 4 */                   \
	ROW(0x7E407C02423C0000), /* 5 */                   \
	ROW(0x3C407C42423C0000), /* 6 */                   \
	ROW(0x7E04081020400000), /* 7 */                   \
	ROW(0x3C423C42423C0000), /* 8 */                   \
	ROW(0x3C42423E023C0000), /* 9 */                   \
	ROW(0x0000080000080000), /* : */                   \
	ROW(0x0000080000081000), /* ; */                   \
	ROW(0x0006186018060000), /* < */                   \
	ROW(0x00007E007E000000), /* = */                   \
	ROW(0x0060180618600000), /* > */                   \
	ROW(0x3844041800100000), /* ? */                   \
	ROW(0x003C449C945C201C), /* @ */ /* 0x40 */        \
	ROW(0x1818243C42420000), /* A */                   \
	ROW(0x7844784444780000), /* B */                   \
	ROW(0x3844808044380000), /* C */                   \
	ROW(0x7844444444780000), /* D */                   \
	ROW(0x7C407840407C0000), /* E */                   \
	ROW(0x7C40784040400000), /* F */                   \
	ROW(0x3844809C44380000), /* G */                   \
	ROW(0x42427E4242420000), /* H */                   \
	ROW(0x3E080808083E0000), /* I */                   \
	ROW(0x1C04040444380000), /* J */                   \
	ROW(0x4448507048440000), /* K */                   \
	ROW(0x40404040407E0000), /* L */                   \
	ROW(0x4163554941410000), /* M */                   \
	ROW(0x4262524A46420000), /* N */                   \
	ROW(0x1C222222221C0000), /* O */                   \
	ROW(0x7844784040400000), /* P */ /* 0x50 */        \
	ROW(0x1C222222221C0200), /* Q */                   \
	ROW(0x7844785048440000), /* R */                   \
	ROW(0x1C22100C221C0000), /* S */                   \
	ROW(0x7F08080808080000), /* T */                   \
	ROW(0x42424242423C0000), /* U */                   \
	ROW(0x8142422424180000), /* V */                   \
	ROW(0x4141495563410000), /* W */                   \
	ROW(0x4224181824420000), /* X */                   \
	ROW(0x4122140808080000), /* Y */                   \
	ROW(0x7E040810207E0000), /* Z */                   \
	ROW(0x3820202020380000), /* [ */                   \
	ROW(0x4020100804020000), /* \ */                   \
	ROW(0x3808080808380000), /* ] */                   \
	ROW(0x1028000000000000), /* ^ */                   \
	ROW(0x00000000007E0000), /* _ */                   \
	ROW(0x1008000000000000), /* ` */ /* 0x60 */        \
	ROW(0x003C023E463A0000), /* a */                   \
	ROW(0x40407C42625C0000), /* b */                   \
	ROW(0x00001C20201C0000), /* c */                   \
	ROW(0x02023E42463A0000), /* d */                   \
	ROW(0x003C427E403C0000), /* e */                   \
	ROW(0x0018103810100000), /* f */                   \
	ROW(0x0000344C44340438), /* g */                   \
	ROW(0x2020382424240000), /* h */                   \
	ROW(0x0800080808080000), /* i */                   \
	ROW(0x0800180808080870), /* j */                   \
	ROW(0x20202428302C0000), /* k */                   \
	ROW(0x1010101010180000), /* l */                   \
	ROW(0x0000665A42420000), /* m */                   \
	ROW(0x00002E3222220000), /* n */                   \
	ROW(0x00003C42423C0000), /* o */                   \
	ROW(0x00005C62427C4040), /* p */ /* 0x70 */        \
	ROW(0x00003A46423E0202), /* q */                   \
	ROW(0x00002C3220200000), /* r */                   \
	ROW(0x001C201804380000), /* s */                   \
	ROW(0x00103C1010180000), /* t */                   \
	ROW(0x00002222261A0000), /* u */                   \
	ROW(0x0000424224180000), /* v */                   \
	ROW(0x000081815A660000), /* w */                   \
	ROW(0x0000422418660000), /* x */                   \
	ROW(0x0000422214081060), /* y */                   \
	ROW(0x00003C08103C0000), /* z */                   \
	ROW(0x1C103030101C0000), /* { */                   \
	ROW(0x0808080808080800), /* | */                   \
	ROW(0x38080C0C08380000), /* } */                   \
	ROW(0x000000324C000000), /* ~ */
/* End of Table */


#define DECLARE_FONT(font_name) \
const uint64_t font_name[NB_FONT_TILES] = { \
	FONT_TABLE \
};

#endif /* FONT_H */
