ECHO = echo
DIRS = RecurrenceSum
all: $(DIRS)

RecurrenceSum: force_look
	cd RecurrenceSum; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
