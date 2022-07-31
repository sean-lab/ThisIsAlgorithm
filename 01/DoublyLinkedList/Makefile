.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = DoublyLinkedList.o Test_DoublyLinkedList.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o DoublyLinkedList $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) DoublyLinkedList test
