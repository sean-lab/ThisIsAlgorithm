.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = CircularDoublyLinkedList.o Test_CircularDoublyLinkedList.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o CircularDoublyLinkedList $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) CircularDoublyLinkedList test
