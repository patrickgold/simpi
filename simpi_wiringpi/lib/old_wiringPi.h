/*!wiringPi.h
 * Header File for wiringPi simulation.
 * 
 * Author: Patrick Goldinger
 * License: MIT
 */

// Pin modes

#define	INPUT               0
#define	OUTPUT              1
#define	PWM_OUTPUT          2
#define	GPIO_CLOCK          3
#define	SOFT_PWM_OUTPUT     4
#define	SOFT_TONE_OUTPUT    5
#define	PWM_TONE_OUTPUT     6

#define	LOW                 0
#define	HIGH                1

// Pull up/down/none

#define	PUD_OFF             0
#define	PUD_DOWN            1
#define	PUD_UP              2

// PWM

#define	PWM_MODE_MS         0
#define	PWM_MODE_BAL        1

// Interrupt levels

#define	INT_EDGE_SETUP      0
#define	INT_EDGE_FALLING    1
#define	INT_EDGE_RISING     2
#define	INT_EDGE_BOTH       3

// Functions

void wiringPiSetupGpio(void);

void pinMode(int pin, int mode);

void pullUpDnControl(int pin, int pud);

void digitalWrite(int pin, int value);

void pwmWrite(int pin, int value);

int digitalRead(int pin);

int analogRead(int pin);

int analogWrite(int pin, int value);

// Extras from arduino land

void         delay             (unsigned int how_long_ms);
void         delayMicroseconds (unsigned int how_long);
unsigned int millis            (void) ;
unsigned int micros            (void) ;
