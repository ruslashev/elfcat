var color = "#ee9";

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
    var primaryElem = document.getElementById(primaryId);
    var secondaryElem = document.getElementById(secondaryId);

    if (primaryElem === null || secondaryElem === null) {
        return;
    }

    addPairHighlighting(primaryElem, secondaryElem);
    addPairHighlighting(secondaryElem, primaryElem);
}

