/*!wiringPi.c
 * Library Source File for wiringPi simulation.
 * Note: this is the actual "hack" here:
 *       Instead of linking the precompiled library for wiringPi, this source
 *       code is being compiled and used.
 * 
 * Author: Patrick Goldinger
 * License: MIT
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "wiringPi.h"

#define BUFSIZE 512

#if defined(_WIN32) || defined(_WIN64)
    #define popen(c1, c2) _popen(c1, c2)
    #define pclose(c1) _pclose(c1)
#elif defined(__linux__) || defined(__linux) || defined(linux)
    #include <unistd.h>
    #include <sys/socket.h>
    #include <sys/signal.h>
    #include <netinet/in.h>
    #include <netinet/tcp.h>
    #include <fcntl.h>
#endif

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

int __str_starts_with(const char* a, const char* b) {
   if (strncmp(a, b, strlen(b)) == 0) return 1;
   return 0;
}

int sock;
struct sockaddr_in server;
char message[256];
char server_reply[1024];

int __recv(int __fd, void *__buf, size_t __n, int __flags) {
    size_t br = 0, br_temp;
    while (br < __n) {
        br_temp = recv(__fd, __buf + br, 1, __flags);
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
    struct __ret_data_t ret = {
        -1, "", "fail", "-1"
    };

    // Create socket
    sock = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (sock < 0) {
        fputs("[ERR] wiringPiSim: Failed to create socket.\n", stderr);
        return ret;
    }

    int nd = 1;
    if (setsockopt(sock, IPPROTO_TCP, TCP_NODELAY, &nd, sizeof(nd)) < 0) {
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
    close(sock);

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
    // char buf[BUFSIZE];
    // FILE *fp;

    // if ((fp = popen(cmd, "r")) == NULL) {
    //     #ifdef DEBUG
    //     printf("Error opening pipe!\n");
    //     #endif
    //     return ret;
    // }

    // while (fgets(buf, BUFSIZE, fp) != NULL) {
    //     char* value = buf;
    //     if (__str_starts_with(buf, "state")) {
    //         value += strlen("state") + 1;
    //         strcpy(ret.state, value);
    //     } else if (__str_starts_with(buf, "value")) {
    //         value += strlen("value") + 1;
    //         strcpy(ret.value, value);
    //     } else if (__str_starts_with(buf, "pin_name")) {
    //         value += strlen("pin_name") + 1;
    //         strcpy(ret.pin_name, value);
    //     } else if (__str_starts_with(buf, "pin_number")) {
    //         value += strlen("pin_number") + 1;
    //         ret.pin_number = atoi(value);
    //     }
    // }

    // if (pclose(fp)) {
    //     #ifdef DEBUG
    //     printf("Command not found or exited with error status\n");
    //     #endif
    //     return ret;
    // }
    //printf("strlen value:%d|%d\n", strlen(ret.value), ret.value[0]);
    return ret;
}

int wiringPiSetupGpio(void) {
    return 0;
}

void pinMode(int pin, int pud) {
    // do nothing
}

void digitalWrite(int pin, int value) {
    char pin_name[16];
    sprintf(pin_name, "GPIO%d", pin);
    char req_str_f[BUFSIZE];
    sprintf(req_str_f, PATH_SET, pin_name, value);
    __request(req_str_f);
}

int digitalRead(int pin) {
    char pin_name[16];
    sprintf(pin_name, "GPIO%d", pin);
    char req_str_f[BUFSIZE];
    sprintf(req_str_f, PATH_GET, pin_name);
    struct __ret_data_t ret = __request(req_str_f);
    return (strcmp(ret.value, "HIGH") == 0) || (strcmp(ret.value, "1") == 0);
}

void delay(unsigned int how_long_ms) {
#if defined(_WIN32) || defined(_WIN64)
    clock_t start_time = clock();
    while (clock() < (start_time + how_long_ms));
#elif defined(__linux__) || defined(__linux) || defined(linux)
    usleep(how_long_ms * 1000);
#endif
}
