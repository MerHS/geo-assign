CXXFLAGS = -Wall -O2 -std=c++11
LDLIBS = -lGL -lGLU -lglut -lm

OBJ := main.o curve.o

bezier: $(OBJ)
	$(LINK.o) $^ $(LDLIBS) -o $@

main.o: curve.o
curve.o: curve.h

.PHONY: clean
clean:
	$(RM) $(OBJ)
