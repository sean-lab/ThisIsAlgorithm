.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BinarySearch.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BinarySearch $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BinarySearch test
