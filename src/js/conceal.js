const prefix = 'bin_';
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

document.addEventListener("mouseover", function (e) {
    let event = e || window.event;
    let target = event.target || event.srcElement;
    let parents = listOfParents(target);
    let cleared = false;

    for (let i = 0; i < parents.length; i++) {
        let id = parents[i];

        if (!id.startsWith(prefix)) {
            continue;
        }

        let tableId = id.replace(prefix, "info_");
        let table = document.getElementById(tableId);

        if (table === null) {
            continue;
        }

        if (!cleared) {
            cleared = true;
            hidePreviousTables();
        }

        table.style.display = "block";

        prevShownTables.push(table);
    }
}, false);
