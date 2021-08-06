function populateOffsets(columns) {
    let rows = Math.ceil(fileLen / columns);
    let elements = "";
    let offset = 0;

    for (let i = 0; i < rows; ++i) {
        elements += offset.toString(16) + "</br>\n";
        offset += columns;
    }

    document.getElementById('offsets').innerHTML = elements;
}
