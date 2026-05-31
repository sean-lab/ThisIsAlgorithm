.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = LCSDC.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LCSDC $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LCSDC test
