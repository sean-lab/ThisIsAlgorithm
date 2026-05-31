.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = FibonacciDP.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o FibonacciDP $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) FibonacciDP test
