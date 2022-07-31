.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = LinkedList.o SequentialSearch.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o SequentialSearch $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LinkedList test
