.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = LinkedList.o Test_LinkedList.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LinkedList $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LinkedList test
