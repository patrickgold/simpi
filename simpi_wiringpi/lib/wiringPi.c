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

#define BUFSIZE 128

#if defined(_WIN32) || defined(_WIN64)
    #define popen(c1, c2) _popen(c1, c2)
    #define pclose(c1) _pclose(c1)
#endif

#define COMMAND "curl -s "
#define HOST "http://127.0.0.1:32000"
#define PATH_GET "/api/getpin/%s"
#define PATH_SET "/api/setpin/%s=%s"
#define CHP_GET COMMAND HOST PATH_GET
#define CHP_SET COMMAND HOST PATH_SET

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

struct __ret_data_t __request(char *cmd) {
    struct __ret_data_t ret = {
        -1, "", "fail", "-1"
    };
    char buf[BUFSIZE];
    FILE *fp;

    if ((fp = popen(cmd, "r")) == NULL) {
        #ifdef DEBUG
        printf("Error opening pipe!\n");
        #endif
        return ret;
    }

    while (fgets(buf, BUFSIZE, fp) != NULL) {
        char* value = buf;
        if (__str_starts_with(buf, "state")) {
            value += strlen("state") + 1;
            strcpy(ret.state, value);
        } else if (__str_starts_with(buf, "value")) {
            value += strlen("value") + 1;
            strcpy(ret.value, value);
        } else if (__str_starts_with(buf, "pin_name")) {
            value += strlen("pin_name") + 1;
            strcpy(ret.pin_name, value);
        } else if (__str_starts_with(buf, "pin_number")) {
            value += strlen("pin_number") + 1;
            ret.pin_number = atoi(value);
        }
    }

    if (pclose(fp)) {
        #ifdef DEBUG
        printf("Command not found or exited with error status\n");
        #endif
        return ret;
    }

    return ret;
}

int wiringPiSetupGpio(void) {
    return 0;
}

void pinMode(int pin, int pud) {
    // do nothing
}

void digitalWrite(int pin, int value) {
    char pin_ntmp[16];
    itoa(pin, pin_ntmp, 10);
    char pin_vtmp[16];
    itoa(value, pin_vtmp, 10);
    char pin_name[16] = "GPIO";
    strcat(pin_name, pin_ntmp);
    char req_str_f[BUFSIZE];
    sprintf(req_str_f, CHP_SET, pin_name, pin_vtmp);
    __request(req_str_f);
}

int digitalRead(int pin) {
    char pin_ntmp[16];
    itoa(pin, pin_ntmp, 10);
    char pin_name[16] = "GPIO";
    strcat(pin_name, pin_ntmp);
    char req_str_f[BUFSIZE];
    sprintf(req_str_f, CHP_GET, pin_name);
    struct __ret_data_t ret = __request(req_str_f);
    return (strcmp(ret.value, "HIGH") == 0) || (strcmp(ret.value, "1") == 0);
}

void delay(unsigned int how_long_ms) {
    clock_t start_time = clock();
    // looping till required time is not acheived 
    while (clock() < (start_time + how_long_ms)); 
}
