let label = document.getElementById('desc')
let descriptions = {
    ehdr:         "ELF file header (ehdr)",
    ident:        "Identifier (e_ident)",
    magic:        "Magic (EI_MAG0 .. EI_MAG3)",
    class:        "Object file class (EI_CLASS)",
    data:         "Data encoding (EI_DATA)",
    ver:          "File version, should be 1 (EI_VERSION)",
    abi:          "ABI (EI_OSABI)",
    abi_ver:      "ABI version, should be 0 for SysV (EI_ABIVERSION)",
    pad:          "Identifier padding",
    e_type:       "Object file type (e_type)",
    e_machine:    "Object file type (e_machine)",
    e_version:    "File version, should be 1 (e_version)",
    e_entry:      "Entrypoint vaddr (e_entry)",
    e_phoff:      "Offset in file to program header table (e_phoff)",
    e_shoff:      "Offset in file to section header table (e_shoff)",
    e_flags:      "Processor-specific flags (e_flags)",
    e_ehsize:     "Size of ELF file header (e_ehsize)",
    e_phentsize:  "Size of a program header table entry (e_phentsize)",
    e_phnum:      "Number of program header table entries (e_phnum)",
    e_shentsize:  "Size of a section header table entry (e_shentsize)",
    e_shnum:      "Number of section header table entries (e_shnum)",
    e_shstrndx:   "What section is a string table (e_shstrndx)",
    phdr:         "Program header",
    p_type:       "Segment type (p_type)",
    p_flags:      "Segment flags (p_flags)",
    p_offset:     "Offset in file (p_offset)",
    p_vaddr:      "Virtual address in memory (p_vaddr)",
    p_paddr:      "Reserved for physical address in memory (p_paddr)",
    p_filesz:     "Size of segment in file (p_filesz)",
    p_memsz:      "Size of segment in memory (p_memsz)",
    p_align:      "Alignment (p_align)",
    segment:      "Segment",
    shdr:         "Section header",
    sh_name:      "Offset to the section name string table containing this section name (sh_name)",
    sh_type:      "Section type (sh_type)",
    sh_flags:     "Section flags (sh_flags)",
    sh_addr:      "Virtual address in memory. Should be 0 if not put into memory (sh_addr)",
    sh_offset:    "Offset in file (sh_offset)",
    sh_size:      "Section size. Also size in file unless type is SHT_NOBITS (sh_size)",
    sh_link:      "Section index of an associated section (sh_link)",
    sh_info:      "Additional information about section (sh_info)",
    sh_addralign: "Address alignment of the section (sh_addralign)",
    sh_entsize:   "Size of each entry if section has table of fixed-size entries (sh_entsize)",
    section:      "Section",
    section_in_segment: "Section in segment",
}
let separator = "<br>&#x2193<br>";

function stripFileInfoPrefix(str) {
    return str.replace("fileinfo_", "");
}

function stripInfoPrefix(str) {
    return str.replace("info_", "");
}

function stripBinPrefix(str) {
    return str.replace("bin_", "");
}

function getDesc(id) {
    if (id === "") {
        return null;
    }

    if (descriptions[id] !== undefined) {
        return descriptions[id];
    }

    if (descriptions[stripFileInfoPrefix(id)] !== undefined) {
        return descriptions[stripFileInfoPrefix(id)];
    }

    if (descriptions[stripInfoPrefix(id)] !== undefined) {
        return descriptions[stripInfoPrefix(id)];
    }

    if (descriptions[stripBinPrefix(id)] !== undefined) {
        return descriptions[stripBinPrefix(id)];
    }

    return null;
}

function iterateParents(el) {
    let txt = "";
    let keywords = [];

    do {
        if (el.id !== undefined) {
            keywords.push(el.id);
        }

        let classList = el.classList;
        if (classList !== undefined) {
            for (let i = 0; i < classList.length; ++i) {
                keywords.push(classList[i]);
            }
        }

        el = el.parentNode;
    } while (el !== null && el.tagName !== "HTML")

    // fix all occurences where segments and sections overlap.
    // once again we are relying on programming by which sections are always inside segments.
    // also, this is O(n^2) ugly.
    let i = 1;
    while (i < keywords.length) {
        if (keywords[i] !== 'segment') {
            ++i;
            continue;
        }

        let have_section_before = false;
        let j = 0;
        for (; j < i; ++j) {
            if (keywords[j] == 'section') {
                have_section_before = true;
                break;
            }
        }

        if (have_section_before) {
            keywords[i] = 'section_in_segment';
            keywords.splice(j, 1);
        } else {
            ++i;
        }
    }

    for (let i = 0; i < keywords.length; ++i) {
        let keyword = keywords[i];
        let desc = getDesc(keyword);

        if (desc !== null) {
            if (txt === "") {
                txt = desc;
            } else {
                txt = desc + separator + txt;
            }
        }
    }

    return txt;
}

document.addEventListener("mouseover", function (e) {
    let event = e || window.event;

    let target = event.target || event.srcElement;

    if (!target.classList.contains('legend_rect')) {
        document.getElementById('desc').innerHTML = iterateParents(target);
    }
}, false);
