.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = PriorityQueue.o Tes_tPriorityQueue.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o PriorityQueue $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) PriorityQueue test
