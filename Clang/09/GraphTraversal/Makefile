.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Graph.o GraphTraversal.o LinkedQueue.o Test_GraphTraversal.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o GraphTraversal $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) GraphTraversal test
