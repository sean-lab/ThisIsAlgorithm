.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = BoyerMoore.o Test_BoyerMoore.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o BoyerMoore $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) BoyerMoore test
