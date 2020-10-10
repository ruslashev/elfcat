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
}
let separator = "</br>&#x2193</br>";

function stripInfoPrefix(str) {
    return str.replace("info_", "");
}

function hasDescription(id) {
    return descriptions[id] !== undefined || descriptions[stripInfoPrefix(id)] !== undefined;
}

function isValid(id) {
    return id !== "" && hasDescription(id);
}

function formatDesc(id) {
    if (descriptions[id] !== undefined) {
        return descriptions[id];
    }

    return descriptions[stripInfoPrefix(id)];
}

function listOfParents(el) {
    var txt = isValid(el.id) ? formatDesc(el.id) : "";

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        if (isValid(el.id)) {
            if (txt === "") {
                txt = formatDesc(el.id);
            } else {
                txt = formatDesc(el.id) + separator + txt;
            }
        }
    }

    return txt;
}

document.onmouseover = function (e) {
    var event = e || window.event;

    var target = event.target || event.srcElement;

    document.getElementById('desc').innerHTML = listOfParents(target);
};
