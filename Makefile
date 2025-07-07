CXX = clang++
CXXFLAGS = -std=c++17 -stdlib=libc++ -g
SRC = main.cpp
OUT = main

build:
	$(CXX) $(CXXFLAGS) $(SRC) -o $(OUT)

run: build
	./$(OUT)
