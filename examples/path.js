const canvas = new Canvas();
const ctx = canvas.getContext('2d');

// canvas.window.title = 'path';

ctx.beginPath();
ctx.moveTo(10,10);
ctx.lineTo(200,100);
ctx.lineTo(200,10);
ctx.lineTo(10,100);
ctx.closePath();
ctx.fill();
