.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi -std=c99
 all: test

OBJS = KarpRabin.o Test_KarpRabin.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o KarpRabin $(OBJS) -lm

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) KarpRabin test
