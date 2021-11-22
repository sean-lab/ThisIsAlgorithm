.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Calculator.o LinkedListStack.o Test_Calculator.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Calculator $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Calculator test
