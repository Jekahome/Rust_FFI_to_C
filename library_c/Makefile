SRC ?= lib.c
TARGET_SRC ?= lib.o
TARGET_DYNAMIC_LIB ?= libfooclib.so
DIAG_FLAGS = -fdiagnostics-color=always -fmessage-length=0 -Wformat-diag

.PHONY: all help

all: compile-gcc create-dynamic-lib

# создаст lib.o из lib.c
compile-gcc:
	gcc -std=c99 -Wall -Wextra -Wformat -Werror -Wconversion -Wformat=2 -Wformat-security $(DIAG_FLAGS) -O0 -fPIC -c $(SRC) -o $(TARGET_SRC)

# создаст libfooclib.so из lib.c
create-dynamic-lib:
	gcc -std=c99 -Wall -Wextra -Wformat -Werror -Wconversion -Wformat=2 -Wformat-security $(DIAG_FLAGS) -O0 -fPIC -shared $(SRC) -o $(TARGET_DYNAMIC_LIB)

help:
	@echo "Use:"
	@echo "  make                  - compile (gcc) and create dynamic library .so"
	@echo "  make SRC=test.c       - compile and run custom C file"
	@echo "  make compile-gcc      - compile using gcc"

