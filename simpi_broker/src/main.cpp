#define DEBUG

// this defines the frame for the game in Windows
#if defined(_WIN32) || defined(_WIN64)
    #define __usingwindows__ 1
    #define STATIC_SERVER_PATH ".\\www\\"
// this defines the frame for the game in Linux
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #define __usinglinux__ 1
    #define STATIC_SERVER_PATH "./www/"
#endif

#include <iostream>
#include "../lib/Broker.hpp"

int main(int argc, char** argv) {
    simpi::Broker broker(STATIC_SERVER_PATH);
    broker.listen("127.0.0.1", 32000);

    std::cout << "Exiting...\n";

    return 0;
}
