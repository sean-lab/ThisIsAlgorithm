ECHO = echo
DIRS = MazeSolver NQueens
all: $(DIRS)

MazeSolver: force_look
	cd MazeSolver; make

NQueens: force_look
	cd NQueens; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
