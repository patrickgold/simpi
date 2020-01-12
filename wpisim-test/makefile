CXX = gcc
CXXFLAGS = -Wall -std=c11 -lpthread

SRC = test/running-light.c
HEADERS = ../wpisim/wiringPi.h
OUT = out/wpi_test

WPISIM_LIB_PATH = ../wpisim/target/debug
WPISIM_LIB = $(WPISIM_LIB_PATH)/libwpisim.so

default: help

# Generates the compiled project
build: $(SRC) $(WPISIM_LIB) $(HEADERS)
	$(CXX) $(SRC) -o $(OUT) -I../wpisim/ -L$(WPISIM_LIB_PATH) -lwpisim $(CXXFLAGS)

run:
	LD_LIBRARY_PATH="$(WPISIM_LIB_PATH)" ./$(OUT)

clean: ./out/*
	rm -rf ./out/*

help:
	@echo "help    - Show this help dialog."
	@echo "build   - Build the executable."
	@echo "clean   - Remove everything from the ./out folder."
	@echo "run     - Run the executable."
