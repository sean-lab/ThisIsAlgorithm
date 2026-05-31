.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = DisjointSet.o Graph.o MST.o PriorityQueue.o Test_MST.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o MinimumSpanningTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) MinimumSpanningTree test
