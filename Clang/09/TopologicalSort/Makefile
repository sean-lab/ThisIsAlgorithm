.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Graph.o LinkedList.o Test_TopologicalSort.o TopologicalSort.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o TopologicalSort $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) TopologicalSort test
