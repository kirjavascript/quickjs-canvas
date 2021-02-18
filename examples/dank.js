x=(new Canvas(550, 500)).getContext('2d');

with (Math) {
    function getR(angle) {
        const r = (1 + 0.9 * cos(8 * angle))
            * (1 + (0.1 * cos(24 * angle)))
            * (0.9 + (0.1 * cos(200 * angle)))
            * (1 + sin(angle));
        return r * 100;
    }
    for (let a = 0.0; a < 6; a += .001) {
        const r = getR(a+4.7);
        const X = (sin(a) * r);
        const Y = (cos(a) * r);
        x.lineTo(X+280, Y+420);
    }
    x.fillStyle='#0A0';
    x.fill();
}
