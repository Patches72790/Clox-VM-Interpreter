CC=gcc
ARGS= -g
OBJS = main.o chunk.o memory.o

run: build
	./clox

build: main.o chunk.o memory.o
	$(CC) $(ARGS) -o clox $(OBJS)

main.o: main.c common.h 
	$(CC) $(ARGS) -c main.c

chunk.o: chunk.c chunk.h
	$(CC) $(ARGS) -c chunk.c

memory.o: memory.c memory.h
	$(CC) $(ARGS) -c memory.c

# creates compilation db for clangd
compiledb:
	compiledb make

clean: 
	rm -rf $(OBJS) clox
