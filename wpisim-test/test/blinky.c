/* --< blinky.c >-- */

#include <stdio.h>
#include <signal.h>
#include "wiringPi.h"

#define LED1 18
#define LED2 23
#define LED3 24
#define LED4 25

#define DELAY_TIME_MS 250

const int LEDS[] = {LED1, LED2, LED3, LED4};

static volatile int keepRunning = 1;

void setup(void);
void blink(int t);
void turnOff(void);
void intHandler(int dummy);

int main() {
    setup();
    while (keepRunning) {
        blink(DELAY_TIME_MS);
    }
    turnOff();
    return 0;
}

void setup() {
    printf("Blinky v0.0.1\n\n");
    signal(SIGINT, intHandler);
    wiringPiSetupGpio();
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        pinMode(LEDS[i], OUTPUT);
    }
}

void blink(int t) {
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        digitalWrite(LEDS[i], LOW);
    }
    delay(t);
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        digitalWrite(LEDS[i], HIGH);
    }
    delay(t);
}

void turnOff() {
    printf("\nCTRL+C detected. Turning all LEDs off and exiting programm...");
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        digitalWrite(LEDS[i], LOW);
    }
}

void intHandler(int dummy) {
    keepRunning = 0;
}
