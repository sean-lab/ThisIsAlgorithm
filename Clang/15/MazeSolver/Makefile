.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = MazeSolver.o Test_MazeSolver.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o MazeSolver $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) MazeSolver test
