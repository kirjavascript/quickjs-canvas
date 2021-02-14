const canvas = new Canvas(500, 400);
const ctx = canvas.getContext('2d');

ctx.fillText('hello world', 32, 48);

const canvas2 = new Canvas();
const ctx2 = canvas2.getContext('2d');

ctx2.fillText('goodbye world', 32, 48-Number.EPSILON);
