ECHO = echo
DIRS = BinaryTree DisjointSet ExpressionTree LCRSTree
all: $(DIRS)

BinaryTree: force_look
	cd BinaryTree; make

DisjointSet: force_look
	cd DisjointSet; make

ExpressionTree: force_look
	cd ExpressionTree; make

LCRSTree: force_look
	cd LCRSTree; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
