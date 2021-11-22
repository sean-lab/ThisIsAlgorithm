.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = Chaining.o Test_Chaining.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o Chaining $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) Chaining test
