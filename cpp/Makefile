all: run

program: main.cpp
	@g++ -std=c++23 -Wall -Wextra -O2 main.cpp -o program

run: program
	@./program

clean:
	@rm -f program
