function listOfParents(el) {
    var list = [el.id];

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        // use unshift() instead of push() because we are stupid and have
        // an assumption that section spans are inside segment spans
        // so that break; in loop below doesn't have to go away.
        list.unshift(el.id);
    }

    return list;
}

var prevTableId;

document.addEventListener("mouseover", function (e) {
    var event = e || window.event;
    var target = event.target || event.srcElement;
    var prefix = 'bin_';

    parents = listOfParents(target);

    for (var i = 0; i < parents.length; i++) {
        var id = parents[i];

        if (id.startsWith(prefix)) {
            var tableId = id.replace(prefix, "info_");
            var target = document.getElementById(tableId);

            if (target !== null) {
                if (prevTableId) {
                    prevTableId.style.display = "none";
                }

                target.style.display = "block";
                prevTableId = target;

                break;
            }
        }
    }
}, false);
