ECHO = echo
DIRS = CircularDoublyLinkedList DoublyLinkedList DoublyLinkedList_Modified LinkedList
all: $(DIRS)

CircularDoublyLinkedList: force_look
	cd CircularDoublyLinkedList; make

DoublyLinkedList: force_look
	cd DoublyLinkedList; make

DoublyLinkedList_Modified: force_look
	cd DoublyLinkedList_Modified; make

LinkedList: force_look
	cd LinkedList; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
