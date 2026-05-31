.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = LCSDP.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o LCSDP $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) LCSDP test
