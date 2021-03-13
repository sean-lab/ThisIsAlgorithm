ECHO = echo
DIRS = BinarySearch BinarySearch2 BinarySearchTree RedBlackTree
all: $(DIRS)

BinarySearch: force_look
	cd BinarySearch; make

BinarySearch2: force_look
	cd BinarySearch2; make

BinarySearchTree: force_look
	cd BinarySearchTree; make

RedBlackTree: force_look
	cd RedBlackTree; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
