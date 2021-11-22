.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BinaryTree.o Test_BinaryTree.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BinaryTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BinaryTree test
