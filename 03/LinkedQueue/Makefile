.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = LinkedQueue.o Test_LinkedQueue.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LinkedQueue $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LinkedQueue test
