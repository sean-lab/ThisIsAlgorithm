.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = InsertionSort.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o InsertionSort $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) InsertionSort test
