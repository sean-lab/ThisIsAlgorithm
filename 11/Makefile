ECHO = echo
DIRS = BoyerMoore BruteForce KarpRabin KnuthMorrisPratt
all: $(DIRS)

BoyerMoore: force_look
	cd BoyerMoore; make

BruteForce: force_look
	cd BruteForce; make

KarpRabin: force_look
	cd KarpRabin; make

KnuthMorrisPratt: force_look
	cd KnuthMorrisPratt; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
