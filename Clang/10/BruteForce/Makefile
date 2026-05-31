.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BruteForce.o Test_BruteForce.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BruteForce $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BruteForce test
