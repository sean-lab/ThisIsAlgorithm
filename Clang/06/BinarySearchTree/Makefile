.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BinarySearchTree.o Test_BinarySearchTree.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BinarySearchTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BinarySearchTree test
