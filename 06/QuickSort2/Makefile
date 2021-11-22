.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = QuickSort2.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o QuickSort2 $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) QuickSort2 test
