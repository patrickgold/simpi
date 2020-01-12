/* --< running-light.c >-- */

#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
//#include <pthread.h>
#include "wiringPi.h"

#define LED1 18
#define LED2 23
#define LED3 24
#define LED4 25
#define BTN1 22
#define BTN2 27
#define BTN3 17

#define INIT_DELAY_TIME_MS 450
#define MAX_DELAY_TIME_MS 950
#define MIN_DELAY_TIME_MS 50
#define STEP_DELAY_TIME_MS 100

#define LTR 1
#define RTL -1

const int LEDS[] = {LED1, LED2, LED3, LED4};

int direction = 1; // 1 = ltr; -1 = rtl
int delayTime = INIT_DELAY_TIME_MS;
//pthread_t th_input, th_output;

unsigned int counter;

void setup(void);
void runningStep(void);
void turnOff(void);
void sigHandler(int);
void decSpeed(void);
void incSpeed(void);
void changeDirection(void);
void* input(void*);
void* output(void*);

int main() {
    setup();
    while (1) output(NULL);
}

void setup() {
    printf("Running Light v0.0.1\n\n");
    signal(SIGINT, sigHandler);
    wiringPiSetupGpio();
    //pthread_create(&th_input, NULL, input, NULL);
    //pthread_create(&th_output, NULL, output, NULL);
    wiringPiISR(BTN1, INT_EDGE_FALLING, decSpeed);
    wiringPiISR(BTN2, INT_EDGE_FALLING, changeDirection);
    wiringPiISR(BTN3, INT_EDGE_FALLING, incSpeed);
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        pinMode(LEDS[i], OUTPUT);
    }
}

void* input(void* args) {
    char c;
    while (1) {
	c = 0;
	c = getchar();
	if (c == '+') {
	    incSpeed();
	} else if (c == '-') {
	    decSpeed();
	} else if (c == '#') {
	    changeDirection();
	}
    }
    
    return NULL;
}
void* output(void* args) {
    while (1) {
	runningStep();
    }
    
    return NULL;
}

void runningStep() {
    int d = direction;
    int t = delayTime;
    printf("delayTime = %3d ms | direction = %2d\r", delayTime, d);
    fflush(stdout);
	
    counter += d;
		
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
	if (counter % 4 == i) {
	    digitalWrite(LEDS[i], HIGH);
	} else {
	    digitalWrite(LEDS[i], LOW);
	}
    }
    
    delay(t);
}

void turnOff() {
    printf("\n\nCTRL+C pressed. Turning all LEDs off and exiting programm...\n");
    //pthread_kill(th_input, NULL);
    //pthread_kill(th_output, NULL);
    for (int i = 0; i < (sizeof(LEDS) / sizeof(const int)); i++) {
        digitalWrite(LEDS[i], LOW);
    }
    delay(100); // wait for low writes to complete
}

void sigHandler(int sig) {
    if (sig != SIGINT) {
	return;
    } else {
	turnOff();
	exit(EXIT_SUCCESS);
    }
}

void changeDirection(void) {
    direction = direction == LTR ? RTL : LTR;
}

void decSpeed(void) {
    if (delayTime >= MAX_DELAY_TIME_MS) {
	delayTime = MAX_DELAY_TIME_MS;
    } else {
	delayTime += STEP_DELAY_TIME_MS;
    }
}
void incSpeed(void) {
    if (delayTime <= MIN_DELAY_TIME_MS) {
	delayTime = MIN_DELAY_TIME_MS;
    } else {
	delayTime -= STEP_DELAY_TIME_MS;
    }
}
