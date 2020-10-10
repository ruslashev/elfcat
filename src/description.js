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
}
let separator = "</br>&#x2193</br>";

function isValid(id) {
    return id !== "" && descriptions[id] !== undefined;
}

function formatDesc(id) {
    return descriptions[id];
}

function listOfParents(el) {
    var txt = isValid(el.id) ? formatDesc(el.id) : "";

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        if (isValid(el.id)) {
            txt = formatDesc(el.id) + separator + txt;
        }
    }

    return txt;
}

// var prevElem;

document.onmouseover = function (e) {
    var event = e || window.event;

    // if (prevElem) {
    //     prevElem.style.border = "";
    // }

    var target = event.target || event.srcElement;

    document.getElementById('desc').innerHTML = listOfParents(target);

    // target.style.border = "1px solid";
    // prevElem = target;
};
