.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = NQueens.o Test_NQueens.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o NQueens $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) NQueens test
