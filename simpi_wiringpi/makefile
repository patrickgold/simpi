CXX = gcc
CXXFLAGS = -Wall -std=c11

WPI_SRC = lib/wiringPi.c
WPI_HEADERS = lib/wiringPi.h
WPI_OUT = out/wiringPiSim.o

SRC = test/blinky.c
HEADERS = test/wiringPi.h
OUT = out/wpi_test

default: help

# Generates the wiringPiSim.o file
build-wiring-pi-sim: $(WPI_SRC) $(WPI_HEADERS)
	$(CXX) $(WPI_SRC) -o $(WPI_OUT) -c $(CXXFLAGS)

# Generates the compiled project
build: $(SRC) $(WPI_OUT) $(HEADERS)
	$(CXX) $(SRC) $(WPI_OUT) -o $(OUT) $(CXXFLAGS)

run:
	./$(OUT)

clean: ./out/*
	rm -r ./out/*

help:
	@echo help    - Show this help dialog.
	@echo build   - Build the executable.
	@echo build-wiring-pi-sim - Build the static .o file for linking.
	@echo clean   - Remove everything from the ./out folder.
	@echo run     - Run the executable.
