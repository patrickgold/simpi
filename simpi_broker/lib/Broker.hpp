/*!Broker.hpp
 * Header File for SimPi Broker.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#ifndef _SIMPI_BROKER_HPP_
#define _SIMPI_BROKER_HPP_

#include <string>
#include "httplib.h"

namespace simpi {

class Broker {
    public:
    Broker(
        std::string static_dir_path,
        std::string prefs_path
    );
    bool listen(const char* host, int port);
    int get_broker_status(void);
    protected:
    std::string _action(std::string cmd);
    private:
    httplib::Server __svr;
    std::string __prefs_path;
    int __broker_status_code;
    std::string __get_app_data_dir();
};

}

#endif // _SIMPI_BROKER_HPP_
