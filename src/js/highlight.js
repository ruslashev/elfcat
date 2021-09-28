let color = "#ee9";

function addPairHighlighting(elem, depElem) {
    elem.addEventListener("mouseenter", function(event) {
        event.target.style.backgroundColor = color;
        depElem.style.backgroundColor = color;
    }, false);

    elem.addEventListener("mouseleave", function(event) {
        event.target.style.backgroundColor = "";
        depElem.style.backgroundColor = "";
    }, false);
}

function highlightIds(primaryId, secondaryId) {
    let primaryElems = document.getElementsByClassName(primaryId);
    let secondaryElems = document.getElementsByClassName(secondaryId);

    if (primaryElems.length === 0 || secondaryElems.length === 0) {
        return;
    }

    let primaryElem = primaryElems[0];
    let secondaryElem = secondaryElems[0];

    addPairHighlighting(primaryElem, secondaryElem);
    addPairHighlighting(secondaryElem, primaryElem);
}

