const canvas = new Canvas();
const ctx = canvas.getContext('2d');

canvas.window.title = 'keyboard';

canvas.addEventListener('keydown', e => {
    console.log(e.key);
});

canvas.addEventListener('mousemove', e => {
    // ctx.fillText('asd', 0, 50);
    console.log(e);
    canvas.window.title = [e.x, e.y];
})
