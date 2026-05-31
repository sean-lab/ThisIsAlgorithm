.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = FibonacciDnC.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o FibonacciDnC $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) FibonacciDnC test
