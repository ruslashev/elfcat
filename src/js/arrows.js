const arrows = document.getElementById('arrows');
let connections = [];
let batchElems = '';

function getAbsPosition(elem) {
    let x = 0, y = 0;

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
    let rect = elem.getBoundingClientRect();

    return {
        w: rect.width || elem.offsetWidth,
        h: rect.height || elem.offsetHeight
    };
}

function addSvgArrow(elem1, elem2) {
    let off1 = getAbsPosition(elem1);
    let off2 = getAbsPosition(elem2);

    let bb1 = getBoundingBoxSizes(elem1);

    let x1 = off1.x + bb1.w / 2;
    let y1 = off1.y;

    let x2 = off2.x;
    let y2 = off2.y;

    batchElems += '<line x1="' + x1 + '" y1="' + y1 + '" '
                      + 'x2="' + x2 + '" y2="' + y2 + '"/>';
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
    let elem1 = document.querySelector(sel1);
    let elem2 = document.querySelector(sel2);

    if (elem1 === null || elem2 === null) {
        return;
    }

    connections.push([elem1, elem2]);

    addSvgArrow(elem1, elem2);

    setJumpCallback(elem1, elem2);
    setJumpCallback(elem2, elem1);
}

function clearArrows() {
    batchElems = '';

    arrows.innerHTML = '';
}

function pushArrowElems() {
    arrows.innerHTML = batchElems;
}

function redrawArrows() {
    clearArrows();

    for (let i = 0; i < connections.length; ++i) {
        let conn = connections[i];

        addSvgArrow(conn[0], conn[1]);
    }

    pushArrowElems();
}

window.onresize = function() {
    redrawArrows();
}

