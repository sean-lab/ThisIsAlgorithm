.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = LCRSTree.o Test_LCRSTree.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LCRSTree $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LCRSTree test
