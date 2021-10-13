### 0.1.8

 * Add pretty-printing on errors instead of just panicking
 * Visualize section headers' links to other section headers with arrows

### 0.1.7

 * Fix extra space at the end of main hexdump div

### 0.1.6

 * Show all infotables (little squares with information about segment, section, program header and
   section header) for currently hovered span, not just the first one. This includes program headers
   that reference a segment and section headers that reference sections.

### 0.1.5

 * Fix janky layout on chrome due to word wrapping
 * Fix needing one-character extra width for main hexdump
 * Slightly speedup initial page load

### 0.1.4

 * Fix highlighting segments when highlighting sections inside them, causing ambiguity. This removes
   highlighting from segments altogether until a better solution will be made.
 * Make all relevant numbers (offset, size, etc) have both hexadecimal and decimal representation
   on hover
 * Add settings with configurable arrow opacity
 * Add help with color legend

### 0.1.3

 * Add +1 character to main hexdump width in chars so that Chrome-based browsers won't wrap words
   causing unaligned layout

### 0.1.2

 * Fix badly creating offsets that made large files unusable
 * Redraw arrows on browser resize event
 * Disallow unwanted size increase of some elements on mobiles

### 0.1.1 (yanked)

 * Fix crates.io not showing readme due to an atypical name

### 0.1.0 (yanked)

 * Initial release

