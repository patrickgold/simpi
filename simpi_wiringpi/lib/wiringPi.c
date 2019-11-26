/*!wiringPi.c
 * Library Source File for wiringPi simulation.
 * Note: this is the actual "hack" here:
 *       Instead of linking the precompiled library for wiringPi, this source
 *       code is being compiled and used.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#define _DEFAULT_SOURCE

#define HOST            "127.0.0.1"
#define PORT             32000
#define PORT_STR        "32000"
#define HTTP_HEADER_GET "GET %s HTTP/1.1\r\nHost: " HOST ":" PORT_STR "\r\nAccept: text/*\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"

#include <stdio.h>
#include <stdlib.h>

#include "lsim.h"
#include "wiringPi.h"

lsim_t data;

// this function is needed to pass data pointer to lsim_cleanup() function
// (c's atexit() doesn't allow parameters)
void cleanup(void) {
    lsim_cleanup(&data);
}

// =====
// Actual wiringPi function implementations
// =====

int wiringPiSetupGpio(void) {
    lsim_setup(&data);
    atexit(cleanup);
    return 0;
}

void pinMode(int pin, int pud) {
    if (pin >= data.gpioregs._min_num && pin <= data.gpioregs._max_num) {
        if (pud == INPUT || pud == OUTPUT) {
            int mode = pud == INPUT ? 1 : 0;
            write_pin(pin, mode, &data.gpioregs.config);
            write_pin(pin, 0, &data.gpioregs.pwm);
        } else if (pud == PWM_OUTPUT) {
            write_pin(pin, 0, &data.gpioregs.config);
            write_pin(pin, 1, &data.gpioregs.pwm);
        }
    }
}

void digitalWrite(int pin, int value) {
    if (pin >= data.gpioregs._min_num && pin <= data.gpioregs._max_num) {
        write_pin(pin, value, &data.gpioregs.output);
    }
}

int digitalRead(int pin) {
    if (pin >= data.gpioregs._min_num && pin <= data.gpioregs._max_num) {
        return read_pin(pin, &data.gpioregs.input);
    } else {
        return -1;
    }
}

int wiringPiISR(int pin, int mode, void (*function)(void)) {
    int v_int0 = mode == INT_EDGE_RISING || mode == INT_EDGE_BOTH;
    int v_int1 = mode == INT_EDGE_RISING || mode == INT_EDGE_FALLING;
    write_pin(pin, v_int0, &data.gpioregs.int0);
    write_pin(pin, v_int1, &data.gpioregs.int1);
    write_pin(pin, 1, &data.gpioregs.inten);
    data.isr_functions[pin] = function;
    return 0;
}

void delay(unsigned int howLong) {
#if defined(PLATFORM__WIN32)
    Sleep(howLong);
#elif defined(PLATFORM__LINUX)
    usleep(howLong * 1000);
#endif
}

void delayMicroseconds(unsigned int howLong) {
#if defined(PLATFORM__WIN32)
    Sleep(howLong / 1000 + 1); // + 1 is for safety reasons
#elif defined(PLATFORM__LINUX)
    usleep(howLong);
#endif
}

unsigned int millis(void) {
    return (unsigned int)((get_sys_time() - data.start_time_us) / 1000);
}

unsigned int micros(void) {
    return (unsigned int)(get_sys_time() - data.start_time_us);
}
