.SUFFIXES : .c .o
CC = gcc
CFLAGS = -g -Wall -ansi  -finput-charset=utf-8 -fexec-charset=euc-kr
 all: test

OBJS = MakingChange.o TestMakingChange.o
SRCS = $(OBJS:.o=.c)

test: $(OBJS)
	$(CC) -o MakingChange $(OBJS) 

clean: 
	rm -rf $(OBJS)
distclean: 
	rm -rf $(OBJS) MakingChange test
