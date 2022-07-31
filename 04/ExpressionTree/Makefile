.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = ExpressionTree.o Test_ExpressionTree.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o ExpressionTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) ExpressionTree test
