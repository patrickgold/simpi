/*!wiringPi.c
 * Library Source File for wiringPi simulation.
 * Note: this is the actual "hack" here:
 *       Instead of linking the precompiled library for wiringPi, this source
 *       code is being compiled and used.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#define _DEFAULT_SOURCE

#define HOST            "127.0.0.1"
#define PORT             32000
#define PORT_STR        "32000"
#define HTTP_HEADER_GET "GET %s HTTP/1.1\r\nHost: " HOST ":" PORT_STR "\r\nAccept: text/*\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"

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
    #include <pthread.h>
#endif

#include "gpioregs.h"
#include "wiringPi.h"

typedef struct {
    char status[32];
    char key[32];
    char value[16];
} ret_data_single_t;
typedef struct {
    char operation[32];
    ret_data_single_t data[16];
} ret_data_t;

uint32_t __get_sys_time(void) {
    uint32_t ret_time_us;
#if defined(PLATFORM__WIN32)
    uint64_t ret_time_us_raw;
    FILETIME curr_time;
    // Gets time in 100ns precision (0.1us)
    GetSystemTimePreciseAsFileTime(&curr_time);
    ret_time_us_raw |= curr_time.dwHighDateTime;
    ret_time_us_raw <<= 32;
    ret_time_us_raw |= curr_time.dwLowDateTime;
    // January 1st, 1970 - January 1st, 1601 UTC ~ 369 years
    // or 116444736000000000 us
    ret_time_us = (uint32_t)(ret_time_us_raw - 116444736000000000);
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

ret_data_t __request(char *cmd) {
    int sock;
    struct sockaddr_in server;
    char message[256];
    char server_reply[1024];
    ret_data_t ret = {
        "?", { (ret_data_single_t){ "FAIL", "", "" } }
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
        fputs("[ERR] wiringPiSim: send() to socket failed.\n", stderr);
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
    int dataSetCount = 0;
    while (curLine) {
        char *nextLine = strchr(curLine, '\n');
        if (nextLine) *nextLine = '\0';  // temporarily terminate the current line
        if (__str_starts_with(curLine, ">")) {
            char *curSet = curLine + 1;
            int dataSetSingleCount = 0;
            while (curSet) {
                char *nextSet = strchr(curSet, ';');
                if (nextSet) *nextSet = '\0';
                if (dataSetSingleCount == 0) {
                    strcpy(ret.data[dataSetCount].status, curSet);
                } else if (dataSetSingleCount == 1) {
                    strcpy(ret.data[dataSetCount].key, curSet);
                } else if (dataSetSingleCount == 2) {
                    strcpy(ret.data[dataSetCount].value, curSet);
                }
                if (nextSet) *nextSet = ';';
                curSet = nextSet ? (nextSet+1) : NULL;
                if (dataSetSingleCount++ >= 2) {
                    break;
                }
            }
            dataSetCount++;
        } else if (__str_starts_with(curLine, "op:")) {
            if (strlen(curLine) > 3) {
                curLine += 3;
                strcpy(ret.operation, curLine);
                curLine -= 3;
            }
        }
        if (nextLine) *nextLine = '\n';  // then restore newline-char, just to be tidy    
        curLine = nextLine ? (nextLine+1) : NULL;
        if (dataSetCount > 15) {
            // break to prevent overflow
            break;
        }
    }

    return ret;
}


// =====
// Data Sync module
// =====
gpioregs_t gpioregs;

void _data_sync_once() {
    ret_data_t ret_data;

    // Get input register from broker
    char req_get[512] = "/api/getreg/input";
    ret_data = __request(req_get);
    gpioregs.input = str_to_reg(ret_data.data[0].value);

    // Set output/config/pwm/intrp register on broker
    char req_set_f[512] = "/api/setreg/output=%s;config=%s;pwm=%s;inten=%s;int0=%s;int1=%s";
    char req_set[512];
    char reg1[16], reg2[16], reg3[16], reg4[16], reg5[16], reg6[16];
    reg_to_str(reg1, &gpioregs.output);
    reg_to_str(reg2, &gpioregs.config);
    reg_to_str(reg3, &gpioregs.pwm);
    reg_to_str(reg4, &gpioregs.inten);
    reg_to_str(reg5, &gpioregs.int0);
    reg_to_str(reg6, &gpioregs.int1);
    sprintf(req_set, req_set_f, reg1, reg2, reg3, reg4, reg5, reg6);
    ret_data = __request(req_set);
}

int is_thread_valid = 0;

#if defined(PLATFORM__WIN32)
HANDLE sync_thread;
DWORD WINAPI _data_sync(void* data) {
  while (is_thread_valid) {
      _data_sync_once();
  }
  return 0;
}
void _cleanup(void) {
    is_thread_valid = 0;
    WaitForSingleObject(sync_thread, 1000);
    CloseHandle(sync_thread);
}
#elif defined(PLATFORM__LINUX)
pthread_t sync_thread;
void *_data_sync(void* data) {
  while (is_thread_valid) {
      _data_sync_once();
  }
  return NULL;
}
void _cleanup(void) {
    is_thread_valid = 0;
    pthread_join(sync_thread, NULL);
}
#endif

// =====
// Actual wiringPi function implementations
// =====

uint32_t start_time_us;

int wiringPiSetupGpio(void) {
    reset_gpio_regs(&gpioregs);
    start_time_us = __get_sys_time();
    is_thread_valid = 1;
#if defined(PLATFORM__WIN32)
    sync_thread = CreateThread(NULL, 0, _data_sync, NULL, 0, NULL);
    if (!sync_thread) {
        fputs("[ERR] wiringPiSim: Failed to create thread for data sync.\n", stderr);
        return EXIT_FAILURE;
    }
#elif defined(PLATFORM__LINUX)
    int succ = pthread_create(&sync_thread, NULL, _data_sync, NULL);
    if (succ != 0) {
        fputs("[ERR] wiringPiSim: Failed to create thread for data sync.\n", stderr);
        return EXIT_FAILURE;
    }
#endif
    atexit(_cleanup);
    return EXIT_SUCCESS;
}

void pinMode(int pin, int pud) {
    if (pin >= gpioregs._min_num && pin <= gpioregs._max_num) {
        if (pud == INPUT || pud == OUTPUT) {
            int mode = pud == INPUT ? 1 : 0;
            write_pin(pin, mode, &gpioregs.config);
            write_pin(pin, 0, &gpioregs.pwm);
        } else if (pud == PWM_OUTPUT) {
            write_pin(pin, 0, &gpioregs.config);
            write_pin(pin, 1, &gpioregs.pwm);
        }
    }
}

void digitalWrite(int pin, int value) {
    if (pin >= gpioregs._min_num && pin <= gpioregs._max_num) {
        write_pin(pin, value, &gpioregs.output);
    }
}

int digitalRead(int pin) {
    if (pin >= gpioregs._min_num && pin <= gpioregs._max_num) {
        return read_pin(pin, &gpioregs.input);
    } else {
        return -1;
    }
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
