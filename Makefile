INCLUDES=-Isrc -Ithird_party

CXX=clang++

ifdef NODEBUG
OPTFLAGS=-O3 -ftree-vectorize -march=native -funroll-loops
CFLAGS=-DNDEBUG
LDFLAGS=
else
OPTFLAGS=-g -O1
CFLAGS=-DDEBUG -fsanitize=address -fno-omit-frame-pointer
LDFLAGS=-fsanitize=address -fno-omit-frame-pointer
endif

CFLAGS+=$(OPTFLAGS) -std=c++17 -W -Wall -Wextra -pedantic
LDFLAGS+=$(OPTFLAGS)


LUX_HPP=$(wildcard src/*.hpp)
LUX_CPP=$(wildcard src/*.cpp)
LUX_OBJ=$(patsubst src/%.cpp, bin/%.o, $(LUX_CPP))

TEST_HPP=$(wildcard test/*.hpp)
TEST_CPP=$(wildcard test/*.cpp)
TEST_OBJ=$(patsubst test/%.cpp, bin/%.o, $(TEST_CPP))

.PHONY: all
.DEFAULT: bin/lux

all: bin/lux

src/%.cpp: $(LUX_HPP)

test/%.cpp: $(TEST_HPP)

bin/%.o: src/%.cpp
	@mkdir -p "$(@D)"
	$(CXX) -c $(INCLUDES) $(CFLAGS) -o $@ $< 

bin/%.o: test/%.cpp
	@mkdir -p "$(@D)"
	$(CXX) -c $(INCLUDES) $(CFLAGS) -o $@ $< 

bin/lux: $(LUX_OBJ)
	$(CXX) $(LUX_OBJ) -o bin/lux $(LDFLAGS)

bin/test: $(TEST_OBJ)
	$(CXX) $(TEST_OBJ) -o bin/test_lux $(LDFLAGS)


clean:
	@rm -rf bin

tidy:
	@echo "Running clang-tidy..."
	@for src in $(LUX_HPP) $(LUX_CPP) ; do \
		clang-tidy "$$src" -- -Isrc $(CFLAGS); \
	done
