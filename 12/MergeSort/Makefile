.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = MergeSort.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o MergeSort $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) MergeSort test
