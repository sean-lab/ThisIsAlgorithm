ECHO = echo
DIRS = FastExponentiation FibonacciDnC MergeSort
all: $(DIRS)

FastExponentiation: force_look
	cd FastExponentiation; make

FibonacciDnC: force_look
	cd FibonacciDnC; make

MergeSort: force_look
	cd MergeSort; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
