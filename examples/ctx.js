const canvas = new Canvas();
const ctx = canvas.getContext('2d');

canvas.window.title = 'rectangles';

ctx.fillText('hello\n world', 0, 0);
ctx.fillRect(0, 20, 50, 10);
ctx.strokeRect(0, 40, 50, 10);
