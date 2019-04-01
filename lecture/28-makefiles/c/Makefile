# Common Variables

# The shell recipes should be interpretted in.
SHELL = /bin/sh

# The C compiler to use.
CC = gcc
# Flags for the C compiler.
CFLAGS = -I. -g

# A list of the object files of our program
objects = main.o helpers.o

# The default goal is a `factorial` program. This links object files.
factorial: $(objects)
	$(CC) $(CFLAGS) -o $@ $^

# Each c file is composed into an object file compiled from a source file.
%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

# PHONY rules are ones that do not produce target files.
.PHONY: clean
clean:
	rm -f factorial $(objects)
