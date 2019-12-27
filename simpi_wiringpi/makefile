CXX = gcc
CXXFLAGS = -Wall -std=c11 -lpthread

WPI_SRC = lib/wiringPi.c
WPI_HEADERS = lib/wiringPi.h lib/gpioregs.h
WPI_OUT = out/wiringPiSim.o
WPI_SO_OUT = out/libwiringPiSim.so

SRC = test/blinky.c
HEADERS = test/wiringPi.h
OUT = out/wpi_test

default: help

# Generates the wiringPiSim.o file
build-wiring-pi-sim: $(WPI_SRC) $(WPI_HEADERS)
	@mkdir -p ./out
	$(CXX) -fPIC -g -c $(WPI_SRC) -o $(WPI_OUT) $(CXXFLAGS)
	$(CXX) -shared -Wl,-soname,libwiringPiSim.so -o $(WPI_SO_OUT) $(WPI_OUT) -lc $(CXXFLAGS)

# Generates the compiled project
build: $(SRC) $(WPI_SO_OUT) $(HEADERS)
	$(CXX) $(SRC) -o $(OUT) -lwiringPiSim $(CXXFLAGS)

run:
	./$(OUT)

clean: ./out/*
	rm -rf ./out/*

help:
	@echo "help    - Show this help dialog."
	@echo "build   - Build the executable."
	@echo "build-wiring-pi-sim - Build the static .o file for linking."
	@echo "clean   - Remove everything from the ./out folder."
	@echo "run     - Run the executable."
