function listOfParents(el) {
    var list = [el.id];

    while (el.tagName !== "HTML") {
        el = el.parentNode;

        list.push(el.id);
    }

    return list;
}

var prevTableId;

document.addEventListener("mouseover", function (e) {
    var event = e || window.event;
    var target = event.target || event.srcElement;
    var prefix = 'bin_';

    if (prevTableId) {
        prevTableId.style.display = "none";
    }

    parents = listOfParents(target);

    for (var i = 0; i < parents.length; i++) {
        var id = parents[i];

        if (id.startsWith(prefix)) {
            var tableId = id.replace(prefix, "info_");
            var target = document.getElementById(tableId);

            if (target !== null) {
                target.style.display = "block";
                prevTableId = target;

                break;
            }
        }
    }
}, false);
