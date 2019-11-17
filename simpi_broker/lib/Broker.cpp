/*!GpioRegister.cpp
 * Source Code File for Raspberry Pi GPIO Register simulation.
 * 
 * Author: Patrick Goldinger
 * License: MIT
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
            response = "status:fail\nvalue:-1\nerror_text:Unknown API Call";
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
    std::string response;
    std::string cmd_single;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, cmd_single, ';')) {
        int state = -1;
        int number;
        std::string response_pin_name = "";
        std::string response_pin_number = "";
        std::stringstream iss(cmd_single);
        iss >> number;
        if (iss.fail()) {
            response_pin_name = cmd_single;
            //if (__gpio.hasPin(cmd_single)) {
                Pin *pin = __gpio.pin(cmd_single);
                if (pin != NULL) {
                    state = pin->read();
                    response_pin_number = std::to_string(pin->number);
                }
            //}
        } else {
            response_pin_number = cmd_single;
            //if (__gpio.hasPin(number)) {
                Pin *pin = __gpio.pin(number);
                if (pin != NULL) {
                    state = pin->read();
                    response_pin_name = pin->name;
                }
            //}
        }
        std::string status = "fail";
        if (state > -1) {
            status = "success";
        }
        response += "pin_number:" + response_pin_number + 
                    "\npin_name:" + response_pin_name + 
                    "\nstatus:" + status +
                    "\nvalue:" + std::to_string(state) + "\n\n";
    }
    response.pop_back();
    response.pop_back();
    return response;
}

std::string simpi::Broker::_setpin(std::string cmd) {
    std::string response;
    std::string cmd_single;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, cmd_single, ';')) {
        std::string name = cmd_single.substr(0, cmd_single.find("="));
        std::string value = cmd_single.erase(0, cmd_single.find("=") + 1);
        int value2 = (value == "HIGH") || (value == "1");
        int state = -1;
        int number;
        std::string response_pin_name = "";
        std::string response_pin_number = "";
        std::stringstream iss(name);
        iss >> number;
        if (iss.fail()) {
            response_pin_name = name;
            //if (__gpio.hasPin(name)) {
                Pin *pin = __gpio.pin(name);
                if (pin != NULL) {
                    state = pin->write(value2);
                    response_pin_number = std::to_string(pin->number);
                }
            //}
        } else {
            response_pin_number = name;
            //if (__gpio.hasPin(number)) {
                Pin *pin = __gpio.pin(number);
                if (pin != NULL) {
                    state = pin->write(value2);
                    response_pin_name = pin->name;
                }
            //}
        }
        std::string status = "fail";
        if (state > -1) {
            status = "success";
        }
        response += "pin_number:" + response_pin_number + 
                    "\npin_name:" + response_pin_name + 
                    "\nstatus:" + status +
                    "\nvalue:" + std::to_string(state) + "\n\n";
    }
    response.pop_back();
    response.pop_back();
    return response;
}

std::string simpi::Broker::_getpref(std::string cmd) {
    return "(null)";
}

std::string simpi::Broker::_setpref(std::string cmd) {
    return "(null)";
}

std::string simpi::Broker::_action(std::string cmd) {
    if (cmd == "terminate") {
        __svr.stop();
        return "status:success\nvalue:0\nadd_text:Exiting...";
    } else if (cmd == "reset") {
        __gpio.reset();
        return "status:success\nvalue:0\nadd_text:Reset done.";
    }
    return "(null)";
}
