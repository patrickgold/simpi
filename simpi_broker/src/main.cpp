/*!main.cpp
 * Main entry point for SimPi Broker.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

// this defines the file paths in Windows
#if defined(_WIN32) || defined(_WIN64)
    #define _SUPPRESS_OUTPUT " >nul 2>&1"
    #define _OPEN_URL_CMD "start"
    #define APPDATA_PATH std::string(getenv("APPDATA")) + "\\simpi"
    #define PREFS_FILE "\\preferences.json"
    #define STATIC_SERVER_PATH ".\\www"
// this defines the file paths in GNU/Linux
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #define _SUPPRESS_OUTPUT " > /dev/null 2>&1"
    #define _OPEN_URL_CMD "xdg-open"
    #define APPDATA_PATH std::string(getenv("HOME")) + "/.simpi"
    #define PREFS_FILE "/preferences.json"
    #define STATIC_SERVER_PATH "./www"
#endif

#include <iostream>
#include <cstdlib>
#include "../lib/Broker.hpp"

using namespace simpi;

int main(int argc, char** argv) {
    std::cout << "################" << std::endl;
    std::cout << "# SimPi Broker #" << std::endl;
    std::cout << "# v0.4.3       #" << std::endl;
    std::cout << "################" << std::endl;
    std::cout << std::endl;
    // #TODO: Insert locking mechanism to prevent multiple open instances
    std::cout << "Opening http://127.0.0.1:32000 in your default browser..." << std::endl;
    std::system((_OPEN_URL_CMD + std::string(" http://127.0.0.1:32000")).c_str());
    std::cout << std::endl;

    std::system(("mkdir " + APPDATA_PATH + _SUPPRESS_OUTPUT).c_str());

    simpi::Broker broker(STATIC_SERVER_PATH, APPDATA_PATH + PREFS_FILE);
    broker.listen("127.0.0.1", 32000);

    std::cout << "Exiting...\n";

    return 0;
}
