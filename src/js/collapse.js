var savedBytes = {};

var marker = '..';

function bytesCallback(ev) {
    var elem = ev.target;

    if (elem.innerHTML === marker) {
        elem.innerHTML = savedBytes[elem.id];
    } else {
        elem.innerHTML = marker;
    }

    redrawArrows();

    ev.preventDefault();
    return false;
}

function collapseBytes() {
    var elements = document.querySelectorAll('.segment, .section');

    for (var i = 0; i < elements.length; ++i) {
        var elem = elements[i];

        if (elem.children.length > 0) {
            continue;
        }

        savedBytes[elem.id] = elem.innerHTML;

        elem.innerHTML = marker;

        elem.addEventListener("contextmenu", bytesCallback, false);
    }
}

collapseBytes();
