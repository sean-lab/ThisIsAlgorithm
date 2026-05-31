ECHO = echo
DIRS = BubbleSort InsertionSort QuickSort QuickSort2
all: $(DIRS)

BubbleSort: force_look
	cd BubbleSort; make

InsertionSort: force_look
	cd InsertionSort; make

QuickSort: force_look
	cd QuickSort; make

QuickSort2: force_look
	cd QuickSort2; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
