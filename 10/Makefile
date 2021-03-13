ECHO = echo
DIRS = Dijkstra Graph GraphTraversal MinimumSpanningTree TopologicalSort
all: $(DIRS)

Dijkstra: force_look
	cd Dijkstra; make

Graph: force_look
	cd Graph; make

GraphTraversal: force_look
	cd GraphTraversal; make

MinimumSpanningTree: force_look
	cd MinimumSpanningTree; make

TopologicalSort: force_look
	cd TopologicalSort; make

clean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) clean); done

distclean: force_look
	-for d in $(DIRS); do (cd $$d; $(MAKE) distclean); done

force_look: 
	true
