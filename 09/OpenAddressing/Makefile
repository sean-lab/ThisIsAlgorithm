.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = OpenAddressing.o Test_OpenAddressing.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o OpenAddressing $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) OpenAddressing test
