.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = CircularQueue.o Test_CircularQueue.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o CircularQueue $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) CircularQueue test
