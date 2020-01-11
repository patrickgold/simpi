CXX = gcc
CXXFLAGS = -Wall -std=c11 -lpthread

SRC = test/blinky.c
HEADERS = test/wiringPi.h
OUT = out/wpi_test

default: help

# Generates the compiled project
build: $(SRC) $(WPI_SO_OUT) $(HEADERS)
	$(CXX) $(SRC) -o $(OUT) -L../wpisim/target/debug/ -lwpisim $(CXXFLAGS)

run:
	LD_LIBRARY_PATH="../wpisim/target/debug/" ./$(OUT)

clean: ./out/*
	rm -rf ./out/*

help:
	@echo "help    - Show this help dialog."
	@echo "build   - Build the executable."
	@echo "clean   - Remove everything from the ./out folder."
	@echo "run     - Run the executable."
