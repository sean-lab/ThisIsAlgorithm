.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = LinkedListStack.o Test_LinkedListStack.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LinkedListStack $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LinkedListStack test
