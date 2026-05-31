.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = KnuthMorrisPratt.o Test_KnuthMorrisPratt.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o KnuthMorrisPratt $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) KnuthMorrisPratt test
