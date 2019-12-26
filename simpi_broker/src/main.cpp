/*!main.cpp
 * Main entry point for SimPi Broker.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

// this defines the file paths in Windows
#if defined(_WIN32) || defined(_WIN64)
    #define APPDATA_PATH std::string(getenv("APPDATA")) + "\\simpi"
    #define PREFS_FILE "\\preferences.json"
    #define STATIC_SERVER_PATH ".\\www\\"
// this defines the file paths in GNU/Linux
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #define APPDATA_PATH std::string(getenv("HOME")) + "/.simpi"
    #define PREFS_FILE "/preferences.json"
    #define STATIC_SERVER_PATH "./www/"
#endif

#include <iostream>
#include "../lib/Broker.hpp"

int main(int argc, char** argv) {
    simpi::Broker broker(STATIC_SERVER_PATH, APPDATA_PATH + PREFS_FILE);
    broker.listen("127.0.0.1", 32000);

    std::cout << "Exiting...\n";

    return 0;
}
