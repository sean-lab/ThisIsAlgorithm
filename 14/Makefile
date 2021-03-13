ECHO = echo
DIRS = FibonacciDP LCSDC LCSDP
all: $(DIRS)

FibonacciDP: force_look
	cd FibonacciDP; make

LCSDC: force_look
	cd LCSDC; make

LCSDP: force_look
	cd LCSDP; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
