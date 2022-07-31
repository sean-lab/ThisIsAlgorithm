ECHO = echo
DIRS = CircularQueue LinkedQueue
all: $(DIRS)

CircularQueue: force_look
	cd CircularQueue; make

LinkedQueue: force_look
	cd LinkedQueue; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
