CXX = g++
CXXFLAGS = -Wall -std=c++17 -lstdc++fs -lpthread

SRC = src/main.cpp lib/Broker.cpp
HEADERS = lib/httplib.h ../simpi_wiringpi/lib/gpioregs.h lib/Broker.hpp
OUT = out/simpi_broker

default: help

build: $(SRC) $(HEADERS)
	@mkdir -p ./out
	$(CXX) $(SRC) -o $(OUT) $(CXXFLAGS)

run:
	./$(OUT)

clean:
	rm -rf ./out/*

help:
	@echo "help    - Show this help dialog."
	@echo "build   - Build the executable."
	@echo "clean   - Remove everything from the ./out folder."
	@echo "run     - Run the executable."
