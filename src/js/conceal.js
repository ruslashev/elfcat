let prevShownTables = [];

function getFirstClass(el) {
    let cls = el.classList[0];

    if (cls === undefined) {
        return "";
    } else {
        return cls;
    }
}

function listOfParents(el) {
    let list = [getFirstClass(el)];

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        list.unshift(getFirstClass(el));
    }

    return list;
}

function hidePreviousTables() {
    for (let i = 0; i < prevShownTables.length; ++i) {
        prevShownTables[i].style.display = "none";
        prevShownTables[i].classList.remove("indirect");
    }
}

function showIfMatches(cl, prefix, newPrefix, indirect = false) {
    if (!cl.startsWith(prefix)) {
        return;
    }

    let tableId = cl.replace(prefix, newPrefix);
    let table = document.getElementById(tableId);

    if (table === null) {
        return;
    }

    table.style.display = "block";

    if (indirect) {
        table.classList.add("indirect");
    } else {
        table.classList.remove("indirect");
    }

    prevShownTables.push(table);
}

document.addEventListener("mouseover", function (e) {
    let event = e || window.event;
    let target = event.target || event.srcElement;
    let parents = listOfParents(target);
    let cleared = false;
    const prefix = 'bin_';

    for (let i = 0; i < parents.length; i++) {
        let cl = parents[i];

        if (!cl.startsWith(prefix)) {
            continue;
        }

        if (!cleared) {
            cleared = true;
            hidePreviousTables();
        }

        showIfMatches(cl, prefix, "info_");
        showIfMatches(cl, "bin_segment", "info_phdr", true);
        showIfMatches(cl, "bin_section", "info_shdr", true);
    }
}, false);
