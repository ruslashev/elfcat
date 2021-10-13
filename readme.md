elfcat - ELF visualizer. Generates HTML files from ELF binaries.

1. How do I install and use it?

       $ cargo install elfcat
       $ elfcat path/to/file

2. What does it look like?

   This is what the following small example ELF file looks like:

   http://ruslashev.github.io/elfcat/hello_world.html

   Steps to create it:

       $ cat hello_world.s
       global _start

       section .text
       _start:
           mov rax, 1
           mov rdi, 1
           mov rsi, msg
           mov rdx, len
           syscall

           mov rax, 60
           xor rdi, rdi
           syscall

       section .data
           msg db "Hello, world!", 0xA
           len equ $ - msg

       $ cat link.ld
       ENTRY(_start)

       SECTIONS {
           . = 0x10080; /* vm.mmap_min_addr + p_offset of first segment */

           .text : {
               * (.text)
           }

           .data : {
               * (.data)
           }
       }

       $ nasm hello_world.s -f elf64
       $ ld hello_world.o -o hello_world -n -T link.ld
       $ elfcat hello_world
       $ xdg-open hello_world.html

3. Can I contribute?

   Of course!

4. License?

   Zlib.

5. Upcoming features?

   * Better text renderer to fix bad performance when opening big files

   * Ability to tune the width instead of hardcoded 16 bytes

   * Visualization of virtual memory mappings

   * Dark theme

   * Highlight bytes in ASCII column

6. Addendum

   * [List of forks](https://github.com/ruslashev/elfcat/wiki/Forks)

