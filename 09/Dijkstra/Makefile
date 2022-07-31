.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Graph.o PriorityQueue.o ShortestPath.o Test_ShortestPath.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Dijkstra $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Dijkstra test
