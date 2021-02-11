.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = Huffman.o PriorityQueue.o Test_Huffman.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Huffman $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Huffman test
