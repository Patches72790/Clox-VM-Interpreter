#include "chunk.h"
#include "common.h"

int main(int argc, char *argv[]) {
    Chunk c;
    initChunk(&c);
    writeChunk(&c, OP_RETURN);
    freeChunk(&c);

  return 0;
}
