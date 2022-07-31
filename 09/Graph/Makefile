.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Graph.o Test_Graph.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Graph $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Graph test
