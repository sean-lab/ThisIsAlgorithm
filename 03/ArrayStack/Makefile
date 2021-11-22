.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = ArrayStack.o Test_ArrayStack.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o ArrayStack $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) ArrayStack test
