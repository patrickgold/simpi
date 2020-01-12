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
#include "httplib.h"

using namespace simpi;

Broker::Broker(
    std::string static_dir_path,
    std::string prefs_path
) : __svr(), __broker_status_code(-1) {
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
        if (std::regex_match(cmd, _, std::regex("action/(.*)"))) {
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

std::string Broker::_action(std::string cmd) {
    std::string op = "op:action\n";
    if (cmd == "terminate") {
        __svr.stop();
        return ">SUCC;terminate;Exiting...\n";
    }
    return op + ">FAIL~UNKACT;" + cmd + ";Invalid action name.\n";
}

std::string Broker::__get_app_data_dir() {
    char *val = getenv("APPDATA");
    return val == NULL ? std::string("") : std::string(val);
}
