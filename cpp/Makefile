all: execute

program: main.cpp
	@g++ -std=c++23 -Wall -Wextra -O2 main.cpp -o program

execute: program
	@./program

run: program
	@for f in in*; do\
		echo "========== Running with $$f =========="; \
		echo ""; \
		./program < $$f; \
		echo ""; \
	done

clean:
	@rm -f program in* out*
