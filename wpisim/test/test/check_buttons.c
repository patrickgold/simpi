/* --< check_buttons.c >-- */

#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include "wiringPi.h"

#define LED1 18
#define LED2 23
#define LED3 24
#define LED4 25
#define BTN1 22
#define BTN2 27
#define BTN3 17

#define DELAY_TIME_MS 100

void check_buttons(int);
int sig_handler(int sig);

int main() {
    printf("Unit Test: check_buttons\n\n");
    signal(SIGINT, sig_handler);
    wiringPiSetupGpio();
    pinMode(BTN1, INPUT);
    pinMode(BTN2, INPUT);
    pinMode(BTN3, INPUT);
    while (1) {
        check_buttons(DELAY_TIME_MS);
    }
    return 0;
}

void check_buttons(int t) {
    int b1 = digitalRead(BTN1);
    int b2 = digitalRead(BTN2);
    int b3 = digitalRead(BTN3);
    printf(
        "BTN1=%d  BTN2=%d  BTN3=%d     \r",
        b1,
        b2,
        b3
    );
    fflush(stdout);
    delay(t);
}

int sig_handler(int sig) {
    printf("\n\nCTRL+C pressed. Terminating...\n");
    exit(EXIT_SUCCESS);
}
