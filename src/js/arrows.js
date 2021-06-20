function getAbsPosition(elem) {
    var x = 0, y = 0;

    do {
        x += elem.offsetLeft || 0;
        y += elem.offsetTop || 0;

        elem = elem.offsetParent;
    } while (elem);

    return {
        x: x,
        y: y
    };
}

function getBoundingBoxSizes(elem) {
    var rect = elem.getBoundingClientRect();

    return {
        w: rect.width || elem.offsetWidth,
        h: rect.height || elem.offsetHeight
    };
}

function addSvgArrow(elem1, elem2) {
    var off1 = getAbsPosition(elem1);
    var off2 = getAbsPosition(elem2);

    var bb1 = getBoundingBoxSizes(elem1);

    var x1 = off1.x + bb1.w / 2;
    var y1 = off1.y;

    var x2 = off2.x;
    var y2 = off2.y;

    document.getElementById('arrows').innerHTML += '<line '
        + 'x1="' + x1 + '" y1="' + y1 + '" '
        + 'x2="' + x2 + '" y2="' + y2 + '" '
        + '/>';
}

function jumpToElem(elem) {
    elem.scrollIntoView();
}

function setJumpCallback(elemFrom, elemTo) {
    elemFrom.addEventListener("click", function(e) {
        jumpToElem(elemTo);
    }, true);
}

function connect(sel1, sel2) {
    var elem1 = document.querySelector(sel1);
    var elem2 = document.querySelector(sel2);

    if (elem1 === null || elem2 === null) {
        return;
    }

    connections.push([elem1, elem2]);

    addSvgArrow(elem1, elem2);

    setJumpCallback(elem1, elem2);
    setJumpCallback(elem2, elem1);
}

var connections = [];

function clearArrows() {
    document.getElementById('arrows').innerHTML = '';
}

function redrawArrows() {
    clearArrows();

    for (var i = 0; i < connections.length; ++i) {
        var conn = connections[i];

        addSvgArrow(conn[0], conn[1]);
    }
}

window.onresize = function() {
    redrawArrows();
}

