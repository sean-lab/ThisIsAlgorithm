ECHO = echo
DIRS = Heap PriorityQueue
all: $(DIRS)

Heap: force_look
	cd Heap; make

PriorityQueue: force_look
	cd PriorityQueue; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
