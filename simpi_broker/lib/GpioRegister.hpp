/*!GpioRegister.hpp
 * Header File for Raspberry Pi GPIO Register simulation.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#ifndef _SIMPI_GPIO_REGISTER_HPP_
#define _SIMPI_GPIO_REGISTER_HPP_

#define _SIMPI_RPI_3BPLUS_PIN_COUNT 40

#include <string>

namespace simpi {

typedef enum {
    UNKNOWN,
    GND,
    DNC,
    CONST_VOLTAGE_3V3,
    CONST_VOLTAGE_5V,
    GPIO,
} pin_type_t;

class Pin {
    public:
    Pin(void);
    Pin(int _number);
    Pin(int _number, pin_type_t _type);
    Pin(int _number, pin_type_t _type, std::string _name);
    Pin(int _number, pin_type_t _type, std::string _name, std::string _name_special);
    int read(void);
    int write(int new_state);
    int number;
    pin_type_t type;
    std::string name;
    std::string name_special;
    private:
    int __state;
};

class GpioRegister {
    public:
    GpioRegister(void);
    bool hasPin(int number);
    bool hasPin(std::string name);
    Pin *pin(int number);
    Pin *pin(std::string name);
    bool reset(void);
    private:
    Pin __gpio_register[_SIMPI_RPI_3BPLUS_PIN_COUNT];
};

}

#endif // _SIMPI_GPIO_REGISTER_HPP_
