ECHO = echo
DIRS = ArrayStack Calculator LinkedListStack
all: $(DIRS)

ArrayStack: force_look
	cd ArrayStack; make

Calculator: force_look
	cd Calculator; make

LinkedListStack: force_look
	cd LinkedListStack; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
