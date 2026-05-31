.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi
 all: test

OBJS = RecurrenceSum.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o RecurrenceSum $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) RecurrenceSum test
