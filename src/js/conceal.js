function listOfParents(el) {
    let list = [el.id];

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        // use unshift() instead of push() because we are stupid and have
        // an assumption that section spans are inside segment spans
        // so that break; in loop below doesn't have to go away.
        list.unshift(el.id);
    }

    return list;
}

let prevTableId;

document.addEventListener("mouseover", function (e) {
    let event = e || window.event;
    let target = event.target || event.srcElement;
    let prefix = 'bin_';

    parents = listOfParents(target);

    for (let i = 0; i < parents.length; i++) {
        let id = parents[i];

        if (!id.startsWith(prefix)) {
            continue;
        }

        let tableId = id.replace(prefix, "info_");
        let target = document.getElementById(tableId);

        if (target === null) {
            continue;
        }

        if (prevTableId) {
            prevTableId.style.display = "none";
        }

        target.style.display = "block";
        prevTableId = target;

        break;
    }
}, false);
