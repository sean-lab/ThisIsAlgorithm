.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = RedBlackTree.o Test_RedBlackTree.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o RedBlackTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) RedBlackTree test
