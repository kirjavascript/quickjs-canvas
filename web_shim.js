function Canvas(width = 300, height = 150) {
    return Object.assign(
        document.body.appendChild(document.createElement('canvas')),
        { width, height },
    );
}
