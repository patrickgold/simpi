/*!GpioRegister.hpp
 * Header File for Raspberry Pi GPIO Register simulation.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#ifndef _SIMPI_BROKER_HPP_
#define _SIMPI_BROKER_HPP_

#include <string>
#include "GpioRegister.hpp"
#include "httplib.h"

namespace simpi {

class Broker {
    public:
    Broker(std::string static_dir_path);
    bool listen(const char* host, int port);
    int get_broker_status(void);
    protected:
    std::string _getpin(std::string cmd);
    std::string _setpin(std::string cmd);
    std::string _getpref(std::string cmd);
    std::string _setpref(std::string cmd);
    std::string _action(std::string cmd);
    private:
    httplib::Server __svr;
    GpioRegister __gpio;
    int __broker_status_code;
};

}

#endif // _SIMPI_BROKER_HPP_
