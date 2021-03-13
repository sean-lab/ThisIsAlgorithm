ECHO = echo
DIRS = Chaining OpenAddressing SimpleHashTable
all: $(DIRS)

Chaining: force_look
	cd Chaining; make

OpenAddressing: force_look
	cd OpenAddressing; make

SimpleHashTable: force_look
	cd SimpleHashTable; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
