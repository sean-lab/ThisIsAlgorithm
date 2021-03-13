.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = DoublyLinkedList.o Test_DoublyLinkedList.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o DoublyLinkedList_Modified $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) DoublyLinkedList_Modified test
