.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BubbleSort.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BubbleSort $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BubbleSort test
