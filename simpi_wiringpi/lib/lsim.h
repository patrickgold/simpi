/*!lsim.h
 * Header File for Library Sim (Base functions for wiringPi.c etc...).
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

#ifndef _SIMPI_LSIM_H_
#define _SIMPI_LSIM_H_

#define _RET_DATA_STR_BUF_SIZE 32
#define _RET_DATA_ARR_BUF_SIZE 16

#include "gpioregs.h"

#if defined(_WIN32) || defined(_WIN64)
    #define PLATFORM__WIN32
    #include <winsock2.h>
    #include <windows.h>
    #pragma comment(lib,"ws2_32.lib") // Winsock Library
    #define __THREAD_TYPE HANDLE
    #define __THREAD_FUNC_RET_TYPE DWORD WINAPI
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
    #define __THREAD_TYPE pthread_t
    #define __THREAD_FUNC_RET_TYPE void *
#endif

#ifdef __cplusplus

#include <cstdio>
#include <cstdlib>
#include <cstdint>
#include <cstring>

namespace simpi {
namespace lsim {

#else

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#endif

typedef struct {
    char status[_RET_DATA_STR_BUF_SIZE];
    char key[_RET_DATA_STR_BUF_SIZE];
    char value[_RET_DATA_STR_BUF_SIZE];
} ret_data_single_t;

typedef struct {
    char operation[_RET_DATA_STR_BUF_SIZE];
    ret_data_single_t data[_RET_DATA_ARR_BUF_SIZE];
} ret_data_t;

typedef struct {
    gpioregs_t gpioregs;
    uint32_t start_time_us;
    int sock_fd;
    struct sockaddr_in server;
    void (*isr_functions[32])(void);
    __THREAD_TYPE sync_thread;
    int is_thread_valid;
} lsim_t;


/*** TIME FUNCTIONS ***/

static inline uint32_t get_sys_time(void) {
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


/*** TEXT FUNCTIONS ***/

static inline int str_starts_with(const char* a, const char* b) {
   if (strncmp(a, b, strlen(b)) == 0) return 1;
   return 0;
}

static inline int parse_simpi_transfer_data(ret_data_t *dst, char *src) {
    char *curLine = src;
    int dataSetCount = 0;
    while (curLine) {
        char *nextLine = strchr(curLine, '\n');
        if (nextLine) *nextLine = '\0';  // temporarily terminate the current line
        if (str_starts_with(curLine, ">")) {
            char *curSet = curLine + 1;
            int dataSetSingleCount = 0;
            while (curSet) {
                char *nextSet = strchr(curSet, ';');
                if (nextSet) *nextSet = '\0';
                if (dataSetSingleCount == 0) {
                    strcpy(dst->data[dataSetCount].status, curSet);
                } else if (dataSetSingleCount == 1) {
                    strcpy(dst->data[dataSetCount].key, curSet);
                } else if (dataSetSingleCount == 2) {
                    strcpy(dst->data[dataSetCount].value, curSet);
                }
                if (nextSet) *nextSet = ';';
                curSet = nextSet ? (nextSet+1) : NULL;
                if (dataSetSingleCount++ >= 2) {
                    break;
                }
            }
            dataSetCount++;
        } else if (str_starts_with(curLine, "op:")) {
            if (strlen(curLine) > 3) {
                curLine += 3;
                strcpy(dst->operation, curLine);
                curLine -= 3;
            }
        }
        if (nextLine) *nextLine = '\n';  // then restore newline-char, just to be tidy    
        curLine = nextLine ? (nextLine+1) : NULL;
        if (dataSetCount > _RET_DATA_ARR_BUF_SIZE) {
            // break to prevent overflow
            break;
        }
    }
    return dataSetCount;
}

static inline int socket_create(int *sock) {
    #if defined(PLATFORM__WIN32)
    WSADATA wsa;
    if (WSAStartup(MAKEWORD(2,2),&wsa) != 0) {
        fprintf(stderr, "[ERR] wiringPiSim: Failed to create socket. (%d)\n", WSAGetLastError());
        return -1;
    }
#endif
    // Create socket
    *sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (*sock < 0) {
        fputs("[ERR] wiringPiSim: Failed to create socket.\n", stderr);
        return -1;
    }
    int nd = 1;
#if defined(PLATFORM__WIN32)
    if (setsockopt(*sock, IPPROTO_TCP, TCP_NODELAY, (char *)&nd, sizeof(nd)) < 0) {
#elif defined(PLATFORM__LINUX)
    if (setsockopt(sock, IPPROTO_TCP, TCP_NODELAY, &nd, sizeof(nd)) < 0) {
#endif
        fputs("[ERR] wiringPiSim: Failed to set socket opt TCP_NODELAY.\n", stderr);
        return -1;
    }
    return 0;
}


/*** SOCKET FUNCTIONS ***/

static inline int socket_init_server_struct(struct sockaddr_in *server, const char *host, int port) {
    server->sin_addr.s_addr = inet_addr(host);
    server->sin_family = AF_INET;
    server->sin_port = htons(port);
    return 0;
}

static inline int socket_connect(int sock, struct sockaddr_in *server) {
    if (connect(sock, (struct sockaddr *)server, sizeof(*server)) < 0) {
        fputs("[ERR] wiringPiSim: Failed to connect to SimPi Broker.\n", stderr);
        return -1;
    }
    return 0;
}

static inline int socket_recv(int __fd, void *__buf, size_t __n, int __flags) {
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

static inline int socket_close(int sock) {
#if defined(PLATFORM__WIN32)
    closesocket(sock);
    WSACleanup();
#elif defined(PLATFORM__LINUX)
    close(sock);
#endif
    return 0;
}


/*** THREAD FUNCTIONS ***/

static inline int thread_create(lsim_t *data, __THREAD_FUNC_RET_TYPE tfunc(void *)) {
#if defined(PLATFORM__WIN32)
    data->sync_thread = CreateThread(NULL, 0, tfunc, data, 0, NULL);
    if (!data->sync_thread) {
        fputs("[ERR] wiringPiSim: Failed to create thread for data sync. Terminating...\n", stderr);
        exit(EXIT_FAILURE);
    }
#elif defined(PLATFORM__LINUX)
    int succ = pthread_create(&data->sync_thread, NULL, tfunc, data);
    if (succ != 0) {
        fputs("[ERR] wiringPiSim: Failed to create thread for data sync. Terminating...\n", stderr);
        exit(EXIT_FAILURE);
    }
#endif
    return 0;
}


/*** DATA SYNC FUNCTIONS ***/

static inline ret_data_t __request(lsim_t *data, char *cmd) {
    int sock;
    char message[256];
    char server_reply[1024];
    ret_data_t ret = {
        "?", { (ret_data_single_t){ "FAIL", "", "" } }
    };

    if (socket_create(&sock) < 0) {
        return ret;
    }

    if (socket_connect(sock, &data->server) < 0) {
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
    if (socket_recv(sock, server_reply, 1024, 0) < 0) {
        fputs("[ERR] wiringPiSim: recv() from socket failed.\n", stderr);
        return ret;
    }
    
    if (socket_close(sock) < 0) {
        return ret;
    }

    char *body = strstr(server_reply, "\r\n\r\n");
    if (body == NULL) {
        fputs("[ERR] wiringPiSim: No valid HTTP response.\n", stderr);
        return ret;
    } else {
        body += 4;
    }

    parse_simpi_transfer_data(&ret, body);

    return ret;
}

static inline int lsim_data_sync_once(lsim_t *data) {
    ret_data_t ret_data;

    // Get input register from broker
    char req_get[512] = "/api/getreg/input";
    ret_data = __request(data, req_get);
    uint32_t old_input = data->gpioregs.input;
    data->gpioregs.input = str_to_reg(ret_data.data[0].value);
    for (int i = data->gpioregs._min_num; i <= data->gpioregs._max_num; i++) {
        if (read_pin(i, &data->gpioregs.inten)) {
            int v_int0 = read_pin(i, &data->gpioregs.int0);
            int v_int1 = read_pin(i, &data->gpioregs.int1);
            int v_inp_old = read_pin(i, &old_input);
            int v_inp_new = read_pin(i, &data->gpioregs.input);
            // rising edge
            if (v_int1 && v_int0
                && !v_inp_old
                && v_inp_new
                && data->isr_functions[i] != NULL
            ) {
                data->isr_functions[i]();
            }
            // falling edge
            else if (v_int1 && !v_int0
                && v_inp_old
                && !v_inp_new
                && data->isr_functions[i] != NULL
            ) {
                data->isr_functions[i]();
            }
            // logical change
            else if (!v_int1 && v_int0
                && (v_inp_old
                ^ v_inp_new)
                && data->isr_functions[i] != NULL
            ) {
                data->isr_functions[i]();
            }
        }
    }

    // Set output/config/pwm/intrp register on broker
    char req_set_f[512] = "/api/setreg/output=%s;config=%s;pwm=%s;inten=%s;int0=%s;int1=%s";
    char req_set[512];
    char reg1[16], reg2[16], reg3[16], reg4[16], reg5[16], reg6[16];
    reg_to_str(reg1, &data->gpioregs.output);
    reg_to_str(reg2, &data->gpioregs.config);
    reg_to_str(reg3, &data->gpioregs.pwm);
    reg_to_str(reg4, &data->gpioregs.inten);
    reg_to_str(reg5, &data->gpioregs.int0);
    reg_to_str(reg6, &data->gpioregs.int1);
    sprintf(req_set, req_set_f, reg1, reg2, reg3, reg4, reg5, reg6);
    ret_data = __request(data, req_set);
    return 0;
}

static inline __THREAD_FUNC_RET_TYPE lsim_data_sync(void* data) {
  while (((lsim_t *)data)->is_thread_valid) {
      lsim_data_sync_once((lsim_t *)data);
  }
  return 0;
}


/*** LSIM FUNCTIONS ***/

static inline int lsim_setup(lsim_t *data) {
    data->start_time_us = get_sys_time();
    socket_init_server_struct(&data->server, HOST, PORT);
    reset_gpio_regs(&data->gpioregs);
    data->is_thread_valid = 1;
    thread_create(data, lsim_data_sync);
    return 0;
}

static inline void lsim_cleanup(lsim_t *data) {
#if defined(PLATFORM__WIN32)
    data->is_thread_valid = 0;
    WaitForSingleObject(data->sync_thread, 1000);
    CloseHandle(data->sync_thread);
#elif defined(PLATFORM__LINUX)
    data->is_thread_valid = 0;
    pthread_join(data->sync_thread, NULL);
#endif
}

#ifdef __cplusplus

// belonging to: namespace simpi { 
// belonging to: namespace lsim {
}
}

#endif

#endif // _SIMPI_LSIM_H_
