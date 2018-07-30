#ifndef _SYS_MMAN_H
#define _SYS_MMAN_H

#include <stdint.h>
#include <sys/types.h>

#define MAP_ANON 4096

#define MAP_FILE 0

#define MAP_FIXED 16

#define MAP_PRIVATE 2

#define MAP_SHARED 1

#define PROT_EXEC 4

#define PROT_NONE 0

#define PROT_READ 1

#define PROT_WRITE 2

void *mmap(void *addr, uintptr_t len, int prot, int flags, int fildes, off_t off);

int munmap(void *addr, uintptr_t len);

#endif /* _SYS_MMAN_H */
