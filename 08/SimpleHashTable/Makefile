.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = SimpleHashTable.o Test_SimpleHashTable.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o SimpleHashTable $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) SimpleHashTable test
