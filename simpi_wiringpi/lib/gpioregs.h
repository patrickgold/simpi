/*!gpioregs.h
 * Header File for Raspberry Pi GPIO Registers.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#ifndef _SIMPI_GPIOREGS_H_
#define _SIMPI_GPIOREGS_H_

#define _RPI_MIN_GPIO_NUM    2
#define _RPI_MAX_GPIO_NUM    27

// =====
// Pin states and min/max values
// Note, that the reg here is 32 bit, but only bits [2;27] are used!
// =====

#ifdef __cplusplus

#include <cstdio>
#include <cstdlib>
#include <cstdint>

namespace simpi {
namespace gpioregs {

#else

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#endif

typedef struct {
    // INPUT register (seen from programmer's view)
    uint32_t input;
    // OUTPUT register (seen from programmer's view)
    uint32_t output;
    // 1=Input 0=Output (All pins default to Input!)
    uint32_t config;
    // 1=PWM 0=No_PWM (ignored if pin in config reg is input!)
    // (currently useless)
    uint32_t pwm;
    // 1=Interrupt 0=No_Interrupt (ignored if pin in config reg is output!)
    // int: 1 0
    //--------------
    //      0 0 ... The low level of the pin generates an interrupt.
    //      0 1 ... Any logical change on the pin generates an interrupt.
    //      1 0 ... The falling edge of the pin generates an interrupt.
    //      1 1 ... The rising edge of the pin generates an interrupt.
    uint32_t inten;
    // Interrupt config bit 2^0 (ignored if pin in inten is disabled!)
    uint32_t int0;
    // Interrupt config bit 2^1 (ignored if pin in inten is disabled!)
    uint32_t int1;
    // min value of gpio num
    const uint8_t _min_num;
    // max value of gpio num
    const uint8_t _max_num;
} gpioregs_t;

static inline void reset_gpio_regs(gpioregs_t *regs) {
    regs->input =   0x00000000;
    regs->output =  0x00000000;
    regs->config =  0xFFFFFFFF;
    regs->pwm =     0x00000000;
    regs->inten =   0x00000000;
    regs->int0 =    0x00000000;
    regs->int1 =    0x00000000;
    *((uint8_t *)(&(regs->_min_num))) = _RPI_MIN_GPIO_NUM;
    *((uint8_t *)(&(regs->_max_num))) = _RPI_MAX_GPIO_NUM;
}
static inline uint8_t read_pin(uint8_t pin, uint32_t *reg) {
    return ((((*reg) >> pin) & 0x1) > 0);
}
static inline void write_pin(uint8_t pin, uint8_t val, uint32_t *reg) {
    if (val > 0) {
        (*reg) |= ((uint32_t)0x1 << pin);
    } else {
        (*reg) &= ~((uint32_t)0x1 << pin);
    }
}
static inline void reg_to_str(char *str, uint32_t *reg) {
    sprintf(str, "0x%.8X", *reg);
}
static inline const uint32_t str_to_reg(const char *str) {
    char *ptr;
    return (uint32_t)strtoul(str, &ptr, 0); // 0 because it is clear (0xAB...)
}

#ifdef __cplusplus

// belonging to: namespace simpi { 
// belonging to: namespace gpioregs {
}
}

#endif

#endif // _SIMPI_GPIOREGS_H_
