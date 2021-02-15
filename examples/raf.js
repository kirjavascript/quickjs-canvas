const canvas = new Canvas();
const ctx = canvas.getContext('2d');

canvas.window.title = 'rAF demo';

let i = 0;

(function loop() {
    requestAnimationFrame(loop);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillText('rAF demo', (Math.cos(i/10) * 50)+100, (Math.sin(i/10) * 50)+75);
    i++;
})();
