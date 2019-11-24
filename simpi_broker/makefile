CXX = g++
CXXFLAGS = -Wall -std=c++17 -lpthread

SRC = src/main.cpp lib/Broker.cpp
HEADERS = lib/httplib.h ../simpi_wiringpi/lib/gpioregs.h lib/Broker.hpp
OUT = out/simpi_broker

default: help

build: $(SRC) $(HEADERS)
	$(CXX) $(SRC) -o $(OUT) $(CXXFLAGS)

run:
	./$(OUT)

clean:
	rm -r ./out/*

help:
	@echo help    - Show this help dialog.
	@echo build   - Build the executable.
	@echo clean   - Remove everything from the ./out folder.
	@echo run     - Run the executable.
