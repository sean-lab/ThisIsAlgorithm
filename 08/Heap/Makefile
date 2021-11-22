.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Heap.o Test_Heap.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Heap $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Heap test
