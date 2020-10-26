let label = document.getElementById('desc')
let descriptions = {
    ehdr: "ELF file header (ehdr)",
    ident: "Identifier (e_ident)",
    magic: "Magic (EI_MAG0 .. EI_MAG3)",
    class: "Object file class (EI_CLASS)",
    data: "Data encoding (EI_DATA)",
    ver: "File version, should be 1 (EI_VERSION)",
    abi: "ABI (EI_OSABI)",
    abi_ver: "ABI version, should be 0 for SysV (EI_ABIVERSION)",
    pad: "Identifier padding",
    e_type: "Object file type (e_type)",
    e_machine: "Object file type (e_machine)",
    e_version: "File version, should be 1 (e_version)",
    e_entry: "Entrypoint vaddr (e_entry)",
    e_phoff: "Offset in file to program header table (e_phoff)",
    e_shoff: "Offset in file to section header table (e_shoff)",
    e_flags: "Processor-specific flags (e_flags)",
    e_ehsize: "Size of ELF file header (e_ehsize)",
    e_phentsize: "Size of a program header table entry (e_phentsize)",
    e_phnum: "Number of program header table entries (e_phnum)",
    e_shentsize: "Size of a section header table entry (e_shentsize)",
    e_shnum: "Number of section header table entries (e_shnum)",
    e_shstrndx: "What section is a string table (e_shstrndx)",
    phdr: "Program header",
    p_type: "Segment type (p_type)",
    p_flags: "Segment flags (p_flags)",
    p_offset: "Offset in file (p_offset)",
    p_vaddr: "Virtual address in memory (p_vaddr)",
    p_paddr: "Reserved for physical address in memory (p_paddr)",
    p_filesz: "Size of segment in file (p_filesz)",
    p_memsz: "Size of segment in memory (p_memsz)",
    p_align: "Alignment (p_align)",
    segment: "Segment",
}
let separator = "<br>&#x2193<br>";

function stripInfoPrefix(str) {
    return str.replace("info_", "");
}

function stripBinPrefix(str) {
    return str.replace("bin_", "");
}

function hasDescription(id) {
    return descriptions[id] !== undefined
        || descriptions[stripInfoPrefix(id)] !== undefined
        || descriptions[stripBinPrefix(id)] !== undefined;
}

function isValid(id) {
    return id !== "" && hasDescription(id);
}

function formatDesc(id) {
    if (descriptions[id] !== undefined) {
        return descriptions[id];
    }

    if (descriptions[stripBinPrefix(id)] !== undefined) {
        return descriptions[stripBinPrefix(id)];
    }

    return descriptions[stripInfoPrefix(id)];
}

function iterateParents(el) {
    var txt = "";

    do {
        var keywords = [];

        if (el.id !== undefined) {
            keywords.push(el.id);
        }

        var classList = el.classList;
        if (classList !== undefined) {
            for (var i = 0; i < classList.length; ++i) {
                keywords.push(classList[i]);
            }
        }

        for (var i = 0; i < keywords.length; i++) {
            var keyword = keywords[i];

            if (isValid(keyword)) {
                if (txt === "") {
                    txt = formatDesc(keyword);
                } else {
                    txt = formatDesc(keyword) + separator + txt;
                }
            }
        }

        el = el.parentNode;
    } while (el !== null && el.tagName !== "HTML")

    return txt;
}

document.addEventListener("mouseover", function (e) {
    var event = e || window.event;

    var target = event.target || event.srcElement;

    document.getElementById('desc').innerHTML = iterateParents(target);
}, false);
