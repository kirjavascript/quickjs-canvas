const canvas = new Canvas();
const ctx = canvas.getContext('2d');

canvas.window.title = 'path';

ctx.beginPath();
ctx.moveTo(20, 100);
ctx.bezierCurveTo(50, 0, 200, 100, 200, 20);
// ctx.lineTo(250, 50);
ctx.lineTo(250, 10);
ctx.closePath()
ctx.fill();

// https://en.wikipedia.org/wiki/Flood_fill
