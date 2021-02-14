const canvas = new Canvas();
const ctx = canvas.getContext('2d');

canvas.window.title = 'fillStyle';

ctx.fillStyle = '#AA2200';

ctx.fillText('hello\n world', 0, 0);

ctx.fillStyle = 'rebeccapurple';
ctx.strokeStyle = 'lime';

assertEq(ctx.fillStyle, '#663399');
assertEq(ctx.strokeStyle, '#00ff00');

ctx.fillStyle = 'invalid style';

assertEq(ctx.fillStyle, '#663399');

ctx.fillRect(0, 20, 50, 10);
ctx.strokeRect(0, 40, 50, 10);


let i = 0;
(function loop() {
    requestAnimationFrame(loop);

    ctx.fillStyle = `hsl(${i%360}deg 100% 50% / 100%)`;
    ctx.fillText('rainbows :3 🌈🌈🌈', 70, 70);

    i++;
})();
