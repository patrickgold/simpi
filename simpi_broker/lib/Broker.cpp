/*!Broker.cpp
 * Source Code File for SimPi Broker.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <regex>
#include "Broker.hpp"
#include "../../simpi_wiringpi/lib/gpioregs.h"
#include "httplib.h"

using namespace simpi;

Broker::Broker(
    std::string static_dir_path,
    std::string prefs_path
) : __svr(), __gpioregs{}, __broker_status_code(-1) {
    gpioregs::reset_gpio_regs(&__gpioregs);
    __prefs_path = prefs_path;

    if (!__svr.is_valid()) {
        std::cout << "Server has an error...\n";
        return;
    }

    __svr.Get("/api/prefs", [&](const httplib::Request &req, httplib::Response &res) {
        //std::cout << __prefs_path << std::endl;
        std::string tmp;
        std::ifstream inp(__prefs_path);
        if (!inp.is_open()) {
            res.set_content("FAIL~IOERROR", "text/plain");
        } else {
            inp >> tmp;
            inp.close();
            res.set_content(tmp, "application/json");
        }
    });

    __svr.Put("/api/prefs", [&](const httplib::Request &req, httplib::Response &res) {
        //std::cout << __prefs_path << std::endl;
        std::ofstream out(__prefs_path);
        if (!out.is_open()) {
            res.set_content("FAIL~IOERROR", "text/plain");
        } else {
            out << req.body;
            out.close();
            res.set_content("SUCC", "text/plain");
        }
    });

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
        } else if (std::regex_match(cmd, _, std::regex("getreg/(.*)"))) {
            response = _getreg(cmd.substr(7));
        } else if (std::regex_match(cmd, _, std::regex("setreg/(.*)"))) {
            response = _setreg(cmd.substr(7));
        } else if (std::regex_match(cmd, _, std::regex("action/(.*)"))) {
            response = _action(cmd.substr(7));
        } else {
            response = "op:" + cmd + "\n>FAIL~UNKAPICALL;;\n";
        }
        res.set_content(response, "text/plain");
    });

    __svr.set_base_dir(static_dir_path.c_str());
}

bool Broker::listen(const char* host, int port) {
    return __svr.listen(host, port);
}

int Broker::get_broker_status() {
    return __broker_status_code;
}

std::string Broker::_getpin(std::string cmd) {
    std::string response = "op:getpin\n";
    std::string name;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, name, ';')) {
        int value_ret = -1;
        int number;
        std::stringstream iss(name);
        iss >> number;
        if (iss.fail()) {
            // thats bad
        } else {
            value_ret = gpioregs::read_pin(number, &__gpioregs.output);
        }
        std::string status = value_ret > -1 ? "SUCC" : "FAIL~PNF";
        response += ">" + status + ";" +
                    name + ";" +
                    std::to_string(value_ret) + "\n";
    }
    return response;
}

std::string Broker::_setpin(std::string cmd) {
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
            // thats bad
        } else {
            gpioregs::write_pin(number, value2, &__gpioregs.input);
            value_ret = value2;
        }
        std::string status = value_ret > -1 ? "SUCC" : "FAIL~PNF";
        response += ">" + status + ";" +
                    name + ";" +
                    std::to_string(value_ret) + "\n";
    }
    return response;
}

std::string Broker::_getreg(std::string cmd) {
    std::string response = "op:getreg\n";
    std::string name;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, name, ';')) {
        uint32_t value_ret = 0x00000000;
        bool is_valid_reg = false;
        if (name == "input") {
            value_ret = __gpioregs.input;
            is_valid_reg = true;
        } else if (name == "output") {
            value_ret = __gpioregs.output;
            is_valid_reg = true;
        } else if (name == "config") {
            value_ret = __gpioregs.config;
            is_valid_reg = true;
        } else if (name == "pwm") {
            value_ret = __gpioregs.pwm;
            is_valid_reg = true;
        } else if (name == "inten") {
            value_ret = __gpioregs.inten;
            is_valid_reg = true;
        } else if (name == "int0") {
            value_ret = __gpioregs.int0;
            is_valid_reg = true;
        } else if (name == "int1") {
            value_ret = __gpioregs.int1;
            is_valid_reg = true;
        }
        char value_ret2[16];
        gpioregs::reg_to_str(value_ret2, &value_ret);
        std::string status = is_valid_reg ? "SUCC" : "FAIL~UNKREG";
        response += ">" + status + ";" +
                    name + ";" +
                    value_ret2 + "\n";
    }
    return response;
}

std::string Broker::_setreg(std::string cmd) {
    std::string response = "op:setreg\n";
    std::string cmd_single;
    std::stringstream scmd(cmd);
    while (std::getline(scmd, cmd_single, ';')) {
        std::string name = cmd_single.substr(0, cmd_single.find("="));
        std::string value = cmd_single.erase(0, cmd_single.find("=") + 1);
        uint32_t value2 = value.size() == 0
            ? 0x00000000 : gpioregs::str_to_reg(value.c_str());
        uint32_t value_ret = 0x00000000;
        bool is_valid_reg = false;
        if (name == "input") {
            value_ret = __gpioregs.input = value2;
            is_valid_reg = true;
        } else if (name == "output") {
            value_ret = __gpioregs.output = value2;
            is_valid_reg = true;
        } else if (name == "config") {
            value_ret = __gpioregs.config = value2;
            is_valid_reg = true;
        } else if (name == "pwm") {
            value_ret = __gpioregs.pwm = value2;
            is_valid_reg = true;
        } else if (name == "inten") {
            value_ret = __gpioregs.inten = value2;
            is_valid_reg = true;
        } else if (name == "int0") {
            value_ret = __gpioregs.int0 = value2;
            is_valid_reg = true;
        } else if (name == "int1") {
            value_ret = __gpioregs.int1 = value2;
            is_valid_reg = true;
        }
        char value_ret2[16];
        gpioregs::reg_to_str(value_ret2, &value_ret);
        std::string status = is_valid_reg ? "SUCC" : "FAIL~UNKREG";
        response += ">" + status + ";" +
                    name + ";" +
                    value_ret2 + "\n";
    }
    return response;
}

std::string Broker::_action(std::string cmd) {
    std::string op = "op:action\n";
    if (cmd == "terminate") {
        __svr.stop();
        return ">SUCC;terminate;Exiting...\n";
    } else if (cmd == "reset") {
        gpioregs::reset_gpio_regs(&__gpioregs);
        return ">SUCC;reset;Reset done.\n";
    }
    return op + ">FAIL~UNKACT;" + cmd + ";Invalid action name.\n";
}

std::string Broker::__get_app_data_dir() {
    char *val = getenv("APPDATA");
    return val == NULL ? std::string("") : std::string(val);
}
