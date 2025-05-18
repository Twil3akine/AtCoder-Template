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

op: program
	@for f in in*; do \
		out=out$${f#in}; \
		echo "========== Generating $$out from $$f =========="; \
		./program < $$f > $$out; \
	done

clean:
	@rm -f program in* out* ./a.out
