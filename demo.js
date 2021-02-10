const canvas = new Canvas();
const ctx = canvas.getContext('2d');

let i = 0;

(function loop() {
    requestAnimationFrame(loop);
    ctx.clearRect(0, 0, 300, 150);
    ctx.fillText('rAF demo', (Math.cos(i/10) * 50)+50, (Math.sin(i/10) * 50)+75);
    i++;

    // TODO: fix slowdown - related to how the UI renders?
})();
