CXX=g++8
ODIR=obj
CXXFLAGS=-O3 -std=c++17 -I../include -L../lib

LIBS=-llanguage_model

main: $(ODIR)/assignment.o main.cpp assignment.hpp
	$(CXX) -o main $(CXXFLAGS) main.cpp $(ODIR)/assignment.o $(LIBS)

$(ODIR)/assignment.o: assignment.cpp assignment.hpp
	$(CXX) -c -o $@ $< $(CXXFLAGS) $(LIBS)

.PHONY: clean

clean:
	rm -f main $(ODIR)/*.o *~ core
