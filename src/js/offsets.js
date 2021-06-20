function populateOffsets(columns) {
    let rows = Math.ceil(fileLen / columns);
    let elements = "";
    var offset = 0;

    for (var i = 0; i < rows; ++i) {
        elements += offset.toString(16) + "</br>\n";
        offset += columns;
    }

    document.getElementById('offsets').innerHTML = elements;
}
