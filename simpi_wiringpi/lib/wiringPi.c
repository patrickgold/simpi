/*!wiringPi.c
 * Library Source File for wiringPi simulation.
 * Note: this is the actual "hack" here:
 *       Instead of linking the precompiled library for wiringPi, this source
 *       code is being compiled and used.
 * 
 * Author: Patrick Goldinger
 * License: MIT
 */

#define _DEFAULT_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#if defined(_WIN32) || defined(_WIN64)
    #define PLATFORM__WIN32
    #include <winsock2.h>
    #include <windows.h>
    #pragma comment(lib,"ws2_32.lib") // Winsock Library
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #define PLATFORM__LINUX
    #include <unistd.h>
    #include <sys/time.h>
    #include <sys/socket.h>
    #include <sys/signal.h>
    #include <netinet/in.h>
    #include <netinet/tcp.h>
    #include <arpa/inet.h>
#endif

#include "wiringPi.h"

#define HTTP_HEADER_GET "GET %s HTTP/1.1\r\nHost: 127.0.0.1:32000\r\nAccept: text/*\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
#define HOST "127.0.0.1"
#define PORT 32000
#define PATH_GET "/api/getpin/%s"
#define PATH_SET "/api/setpin/%s=%d"

struct __ret_data_t {
    int pin_number;
    char pin_name[16];
    char state[16];
    char value[16];
};

uint64_t __get_sys_time(void) {
    uint64_t ret_time_us_raw;
#if defined(PLATFORM__WIN32)
    FILETIME curr_time;
    // Gets time in 100ns precision (0.1us)
    GetSystemTimePreciseAsFileTime(&curr_time);
    ret_time_us_raw |= curr_time.dwHighDateTime;
    ret_time_us_raw <<= 32;
    ret_time_us_raw |= curr_time.dwLowDateTime;
    // January 1st, 1970 - January 1st, 1601 UTC ~ 369 years
    // or 116444736000000000 us
    uint32_t ret_time_us = (uint32_t)(ret_time_us_raw - 116444736000000000);
    ret_time_us /= (uint32_t)10; 
#elif defined(PLATFORM__LINUX)
    struct timeval curr_time;
    gettimeofday(&curr_time, NULL);
    ret_time_us = curr_time.tv_sec * (int)1e6 + curr_time.tv_usec;
#endif
return ret_time_us;
}

int __str_starts_with(const char* a, const char* b) {
   if (strncmp(a, b, strlen(b)) == 0) return 1;
   return 0;
}

int __recv(int __fd, void *__buf, size_t __n, int __flags) {
    size_t br = 0, br_temp;
    while (br < __n) {
        br_temp = recv(__fd, (char *)__buf + br, 1, __flags);
        if (br_temp == 0) {
            return 0;
        } else if (br_temp < 0) {
            return -1;
        }
        br += br_temp;
    }
    *((char *)__buf + br) = 0;
    return 0;
}

struct __ret_data_t __request(char *cmd) {
    int sock;
    struct sockaddr_in server;
    char message[256];
    char server_reply[1024];
    struct __ret_data_t ret = {
        -1, "", "fail", "-1"
    };

#if defined(PLATFORM__WIN32)
    WSADATA wsa;
    if (WSAStartup(MAKEWORD(2,2),&wsa) != 0) {
		fprintf(stderr, "[ERR] wiringPiSim: Failed to create socket. (%d)\n", WSAGetLastError());
		return ret;
	}
#endif

    // Create socket
    sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (sock < 0) {
        fputs("[ERR] wiringPiSim: Failed to create socket.\n", stderr);
        return ret;
    }

    int nd = 1;
#if defined(PLATFORM__WIN32)
    if (setsockopt(sock, IPPROTO_TCP, TCP_NODELAY, (char *)&nd, sizeof(nd)) < 0) {
#elif defined(PLATFORM__LINUX)
    if (setsockopt(sock, IPPROTO_TCP, TCP_NODELAY, &nd, sizeof(nd)) < 0) {
#endif
        fputs("[ERR] wiringPiSim: Failed to set socket opt TCP_NODELAY.\n", stderr);
        return ret;
    }

    server.sin_addr.s_addr = inet_addr(HOST);
    server.sin_family = AF_INET;
    server.sin_port = htons(PORT);

    // Connect to remote server
    if (connect(sock , (struct sockaddr *)&server , sizeof(server)) < 0) {
        fputs("[ERR] wiringPiSim: Failed to connect to SimPi Broker.\n", stderr);
        return ret;
    }

    // Send some data
    sprintf(message, HTTP_HEADER_GET, cmd);
	if (send(sock, message, strlen(message), 0) < 0) {
		fputs("[ERR] wiringPiSim: write() to socket failed.\n", stderr);
		return ret;
	}

    // Receive a reply from the server
    memset(server_reply, 0, 1024);
	if (__recv(sock, server_reply, 1024, 0) < 0) {
		fputs("[ERR] wiringPiSim: recv() from socket failed.\n", stderr);
        return ret;
	}

#if defined(PLATFORM__WIN32)
    closesocket(sock);
    WSACleanup();
#elif defined(PLATFORM__LINUX)
    close(sock);
#endif

    char *body = strstr(server_reply, "\r\n\r\n");
    if (body == NULL) {
		fputs("[ERR] wiringPiSim: No valid HTTP response.\n", stderr);
        return ret;
    } else {
        body += 4;
    }

    char *curLine = body;
    while (curLine) {
        char *nextLine = strchr(curLine, '\n');
        if (nextLine) *nextLine = '\0';  // temporarily terminate the current line
        if (__str_starts_with(curLine, "state")) {
            curLine += strlen("state") + 1;
            strcpy(ret.state, curLine);
        } else if (__str_starts_with(curLine, "value")) {
            curLine += strlen("value") + 1;
            strcpy(ret.value, curLine);
        } else if (__str_starts_with(curLine, "pin_name")) {
            curLine += strlen("pin_name") + 1;
            strcpy(ret.pin_name, curLine);
        } else if (__str_starts_with(curLine, "pin_number")) {
            curLine += strlen("pin_number") + 1;
            ret.pin_number = atoi(curLine);
        }
        if (nextLine) *nextLine = '\n';  // then restore newline-char, just to be tidy    
        curLine = nextLine ? (nextLine+1) : NULL;
    }

    return ret;
}

// =====
// Actual wiringPi function implementations
// =====

uint64_t start_time_us;

int wiringPiSetupGpio(void) {
    start_time_us = __get_sys_time();
    return 0;
}

void pinMode(int pin, int pud) {
    // do nothing
}

void digitalWrite(int pin, int value) {
    char pin_name[16];
    sprintf(pin_name, "GPIO%d", pin);
    char req_str_f[512];
    sprintf(req_str_f, PATH_SET, pin_name, value);
    __request(req_str_f);
}

int digitalRead(int pin) {
    char pin_name[16];
    sprintf(pin_name, "GPIO%d", pin);
    char req_str_f[512];
    sprintf(req_str_f, PATH_GET, pin_name);
    struct __ret_data_t ret = __request(req_str_f);
    return (strcmp(ret.value, "HIGH") == 0) || (strcmp(ret.value, "1") == 0);
}

void delay(unsigned int howLong) {
#if defined(PLATFORM__WIN32)
    Sleep(howLong);
#elif defined(PLATFORM__LINUX)
    usleep(howLong * 1000);
#endif
}

void delayMicroseconds(unsigned int howLong) {
#if defined(PLATFORM__WIN32)
    Sleep(howLong / 1000 + 1); // + 1 is for safety reasons
#elif defined(PLATFORM__LINUX)
    usleep(howLong);
#endif
}

unsigned int millis(void) {
    return (unsigned int)((__get_sys_time() - start_time_us) / 1000);
}

unsigned int micros(void) {
    return (unsigned int)(__get_sys_time() - start_time_us);
}
