elfcat - ELF visualizer. Generates HTML files from ELF binaries.

1. How do I install and use it?

       $ cargo install elfcat
       $ elfcat path/to/file

2. How does it look like?

   This is how the following small example ELF file looks like:

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

5. When I try this on huge files, it slows down my browser!

   Sorry about that. There used to be a feature where segments and sections
   would be collapsed into a couple characters instead of showing full contents,
   but it was disabled because it:

    * broke when these would overlap

    * in a way, defeats the purpose: seeing specific bytes is a feature.

6. Upcoming features?

   * Ability to tune the width instead of hardcoded 16 bytes

   * Visualization of virtual memory mappings

   * Dark theme

   * Highlight bytes in ASCII column

