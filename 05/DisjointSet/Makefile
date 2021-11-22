.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = DisjointSet.o Test_DisjointSet.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o DisjointSet $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) DisjointSet test
