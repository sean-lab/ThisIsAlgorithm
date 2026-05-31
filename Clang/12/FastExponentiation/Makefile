.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = FastExponentiation.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o FastExponentiation $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) FastExponentiation test
