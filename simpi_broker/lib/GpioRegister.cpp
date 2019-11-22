/*!GpioRegister.cpp
 * Source Code File for Raspberry Pi GPIO Register simulation.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#include <string>
#include "GpioRegister.hpp"

#if defined(_WIN32) || defined(_WIN64)
    #define TMP_PATH "simpi\\"
    #define TMP_PATH_LEDS TMP_PATH "leds\\"
    #define TMP_PATH_BTNS TMP_PATH "btns\\"
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #define TMP_PATH "simpi/"
    #define TMP_PATH_LEDS TMP_PATH "leds/"
    #define TMP_PATH_BTNS TMP_PATH "btns/"
#endif

simpi::Pin::Pin(void) : simpi::Pin::Pin(-1) {};

simpi::Pin::Pin(int _number) : simpi::Pin::Pin(_number, UNKNOWN) {};

simpi::Pin::Pin(int _number, simpi::pin_type_t _type) : simpi::Pin::Pin(_number, _type, "") {};

simpi::Pin::Pin(int _number, simpi::pin_type_t _type, std::string _name) : simpi::Pin::Pin(_number, _type, _name, "") {};

simpi::Pin::Pin(int _number, simpi::pin_type_t _type, std::string _name, std::string _name_special) : number(_number), type(_type), name(_name), name_special(_name_special), __state(0) {};

int simpi::Pin::read(void) {
    return __state;
}

int simpi::Pin::write(int new_state) {
    return __state = new_state;
}


simpi::GpioRegister::GpioRegister(void) : __gpio_register {
    Pin(1, CONST_VOLTAGE_3V3, "3V3_1"),
    Pin(2, CONST_VOLTAGE_5V, "5V_1"),
    Pin(3, GPIO, "GPIO2", "SDA"),
    Pin(4, CONST_VOLTAGE_5V, "5V_2"),
    Pin(5, GPIO, "GPIO3", "SCL"),
    Pin(6, GND, "GND_1"),
    Pin(7, GPIO, "GPIO4"),
    Pin(8, GPIO, "GPIO14", "UART0_TXD"),
    Pin(9, GND, "GND_2"),
    Pin(10, GPIO, "GPIO15", "UART0_RXD"),
    Pin(11, GPIO, "GPIO17"),
    Pin(12, GPIO, "GPIO18", "CLK"),
    Pin(13, GPIO, "GPIO27"),
    Pin(14, GND, "GND_3"),
    Pin(15, GPIO, "GPIO22"),
    Pin(16, GPIO, "GPIO23"),
    Pin(17, CONST_VOLTAGE_3V3, "3V3_2"),
    Pin(18, GPIO, "GPIO24"),
    Pin(19, GPIO, "GPIO10", "MOSI"),
    Pin(20, GND, "GND_4"),
    Pin(21, GPIO, "GPIO9", "MISO"),
    Pin(22, GPIO, "GPIO25"),
    Pin(23, GPIO, "GPIO11", "CLK"),
    Pin(24, GPIO, "GPIO8", "CE0_N"),
    Pin(25, GND, "GND_5"),
    Pin(26, GPIO, "GPIO7", "CE1_N"),
    Pin(27, DNC, "DNC_1", "I2C"),
    Pin(28, DNC, "DNC_2", "I2C"),
    Pin(29, GPIO, "GPIO5"),
    Pin(30, GND, "GND_6"),
    Pin(31, GPIO, "GPIO6"),
    Pin(32, GPIO, "GPIO12"),
    Pin(33, GPIO, "GPIO13"),
    Pin(34, GND, "GND_7"),
    Pin(35, GPIO, "GPIO19"),
    Pin(36, GPIO, "GPIO16"),
    Pin(37, GPIO, "GPIO26"),
    Pin(38, GPIO, "GPIO20"),
    Pin(39, GND, "GND_8"),
    Pin(40, GPIO, "GPIO21")
} {};


bool simpi::GpioRegister::hasPin(int number) {
    return number <= 40 && number >= 1;
}

bool simpi::GpioRegister::hasPin(std::string name) {
    for (int i = 0; i < 40; i++) {
        if (__gpio_register[i].name == name) {
            return true;
        }
    }
    return false;
}

simpi::Pin *simpi::GpioRegister::pin(int number) {
    if (hasPin(number)) {
        return &__gpio_register[number - 1];
    } else {
        return NULL;
    }
}

simpi::Pin *simpi::GpioRegister::pin(std::string name) {
    for (int i = 0; i < 40; i++) {
        if (__gpio_register[i].name == name) {
            return &__gpio_register[i];
        }
    }
    return NULL;
}

bool simpi::GpioRegister::reset(void) {
    for (int i = 0; i < 40; i++) {
        __gpio_register[i].write(0);
    }
    return true;
}
