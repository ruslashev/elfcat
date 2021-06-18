function populateOffsets(columns) {
    let rows = Math.ceil(fileLen / columns);
    let container = document.getElementById('offsets');
    var offset = 0;

    for (var i = 0; i < rows; ++i) {
        container.innerHTML += offset.toString(16) + "</br>\n";
        offset += columns;
    }
}
