/*!GpioRegister.cpp
 * Source Code File for Raspberry Pi GPIO Register simulation.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#include <iostream>
#include <sstream>
#include <string>
#include <regex>
#include "Broker.hpp"
#include "GpioRegister.hpp"
#include "httplib.h"

simpi::Broker::Broker(std::string static_dir_path)
    : __svr(), __gpio(), __broker_status_code(-1)
{
    if (!__svr.is_valid()) {
        std::cout << "Server has an error...\n";
        return;
    }

    __svr.Get(R"(/api/(.*))", [&](const httplib::Request &req, httplib::Response &res) {
        //#ifdef DEBUG
        //std::cout << req.target << std::endl;
        //std::cout << req << std::endl;
        //#endif
        std::string cmd = req.target.substr(5);
        std::string response;
        std::smatch _;
        if (std::regex_match(cmd, _, std::regex("getpin/(.*)"))) {
            response = _getpin(cmd.substr(7));
        } else if (std::regex_match(cmd, _, std::regex("setpin/(.*)"))) {
            response = _setpin(cmd.substr(7));
        } else if (std::regex_match(cmd, _, std::regex("getpref/(.*)"))) {
            response = _getpref(cmd.substr(8));
        } else if (std::regex_match(cmd, _, std::regex("setpref/(.*)"))) {
            response = _setpref(cmd.substr(8));
        } else if (std::regex_match(cmd, _, std::regex("action/(.*)"))) {
            response = _action(cmd.substr(7));
        } else {
            response = "op:" + cmd + "\n>FAIL~UNKAPICALL;;\n";
        }
        //res.set_header("X-Content-Type-Options", "nosniff");
        //std::cout << response << std::endl;
        res.set_content(response, "text/plain");
    });

    __svr.set_base_dir(static_dir_path.c_str());
}

bool simpi::Broker::listen(const char* host, int port) {
    return __svr.listen(host, port);
}

int simpi::Broker::get_broker_status() {
    return __broker_status_code;
}

std::string simpi::Broker::_getpin(std::string cmd) {
    std::string response = "op:getpin\n";
    std::string name;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, name, ';')) {
        int value_ret = -1;
        int number;
        std::stringstream iss(name);
        iss >> number;
        if (iss.fail()) {
            //if (__gpio.hasPin(name)) {
                Pin *pin = __gpio.pin(name);
                if (pin != NULL) {
                    value_ret = pin->read();
                }
            //}
        } else {
            //if (__gpio.hasPin(number)) {
                Pin *pin = __gpio.pin(number);
                if (pin != NULL) {
                    value_ret = pin->read();
                }
            //}
        }
        std::string status = "FAIL~PNF";
        if (value_ret > -1) {
            status = "SUCC";
        }
        response += ">" + status + ";" +
                    name + ";" +
                    std::to_string(value_ret) + "\n";
    }
    return response;
}

std::string simpi::Broker::_setpin(std::string cmd) {
    std::string response = "op:setpin\n";
    std::string cmd_single;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, cmd_single, ';')) {
        std::string name = cmd_single.substr(0, cmd_single.find("="));
        std::string value = cmd_single.erase(0, cmd_single.find("=") + 1);
        int value2 = (value == "HIGH") || (value == "1");
        int value_ret = -1;
        int number;
        std::stringstream iss(name);
        iss >> number;
        if (iss.fail()) {
            //if (__gpio.hasPin(name)) {
                Pin *pin = __gpio.pin(name);
                if (pin != NULL) {
                    value_ret = pin->write(value2);
                }
            //}
        } else {
            //if (__gpio.hasPin(number)) {
                Pin *pin = __gpio.pin(number);
                if (pin != NULL) {
                    value_ret = pin->write(value2);
                }
            //}
        }
        std::string status = "FAIL~PNF";
        if (value_ret > -1) {
            status = "SUCC";
        }
        response += ">" + status + ";" +
                    name + ";" +
                    std::to_string(value_ret) + "\n";
    }
    return response;
}

std::string simpi::Broker::_getpref(std::string cmd) {
    return "op:getpref\n>FAIL~NYI;;\n";
}

std::string simpi::Broker::_setpref(std::string cmd) {
    return "op:setpref\n>FAIL~NYI;;\n";
}

std::string simpi::Broker::_action(std::string cmd) {
    std::string op = "op:action\n";
    if (cmd == "terminate") {
        __svr.stop();
        return ">SUCC;terminate;Exiting...\n";
    } else if (cmd == "reset") {
        __gpio.reset();
        return ">SUCC;reset;Reset done.\n";
    }
    return op + ">FAIL~UNKACT;" + cmd + ";Invalid action name.\n";
}
