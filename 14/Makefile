ECHO = echo
DIRS = Huffman MakingChange
all: $(DIRS)

Huffman: force_look
	cd Huffman; make

MakingChange: force_look
	cd MakingChange; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
