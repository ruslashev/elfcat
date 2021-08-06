let prevShownTables = [];

function listOfParents(el) {
    let list = [el.id];

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        list.push(el.id);
    }

    return list;
}

function hidePreviousTables() {
    for (let i = 0; i < prevShownTables.length; ++i) {
        prevShownTables[i].style.display = "none";
    }
}

function showIfMatches(id, prefix, newPrefix) {
    if (!id.startsWith(prefix)) {
        return;
    }

    let tableId = id.replace(prefix, newPrefix);
    let table = document.getElementById(tableId);

    if (table === null) {
        return;
    }

    table.style.display = "block";

    prevShownTables.push(table);
}

document.addEventListener("mouseover", function (e) {
    let event = e || window.event;
    let target = event.target || event.srcElement;
    let parents = listOfParents(target);
    let cleared = false;
    const prefix = 'bin_';

    for (let i = 0; i < parents.length; i++) {
        let id = parents[i];

        if (!id.startsWith(prefix)) {
            continue;
        }

        if (!cleared) {
            cleared = true;
            hidePreviousTables();
        }

        showIfMatches(id, prefix, "info_");
        showIfMatches(id, "bin_segment", "info_phdr");
        showIfMatches(id, "bin_section", "info_shdr");
    }
}, false);
