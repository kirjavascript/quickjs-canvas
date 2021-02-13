function Canvas() {
    return Object.assign(document.body.appendChild(document.createElement('canvas')), { window: {} });
}
