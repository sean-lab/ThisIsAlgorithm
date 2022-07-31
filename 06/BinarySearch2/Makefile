.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BinarySearch2.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BinarySearch2 $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BinarySearch2 test
