'use strict';

(function() {
    // https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement

    const initialTime = Date.now();

    // const float = (num) => num + Number.EPSILON;
    const int = (num) => Math.round(num);

    class HTMLCanvasElement {
        #id;

        constructor(width = 300, height = 150) {
            this.#width = int(width);
            this.#height = int(height);
            this.#id = QJSC_initCanvas(this.#width, this.#height);
        }

        getContext(type) {
            if (type !== '2d') {
                throw new Error(`Currently only 2D rendering is supported`);
            }
            return new CanvasRenderingContext2D(this.#id);
        };

        #window;

        get window() {
            if (!this.#window) {
                this.#window = new Window(this.#id);
            }
            return this.#window;
        }

        #width;
        #height;

        get width() {
            return this.#width;
        }

        get height() {
            return this.#height;
        }

        set width(prop) {
            const width = int(prop)
            if (width >= 0) {
                this.#width = width;
                QJSC_setSize(this.#id, this.#width, this.#height);
            }
        }

        set height(prop) {
            const height = int(prop)
            if (height >= 0) {
                this.#height = height;
                QJSC_setSize(this.#id, this.#width, this.#height);
            }
        }

        #events = new Map();

        addEventListener() {

        }

        removeEventListener() {

        }

        // style {backgroundColor, cursor}

        //toDataURL
        //toBlob
    }

    // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D

    class CanvasRenderingContext2D {
        #id;

        constructor(id) {
            this.#id = id;
        }

        clearRect(x, y, w, h) {
            QJSC_clearRect(this.#id, int(x), int(y), int(w), int(h));
        }
        fillRect(x, y, w, h) {
            QJSC_fillRect(this.#id,  int(x), int(y), int(w), int(h));
        }
        strokeRect(x, y, w, h) {
            QJSC_strokeRect(this.#id,  int(x), int(y), int(w), int(h));
        }

        fillText(text, x, y) {
            QJSC_fillText(this.#id, String(text), int(x), int(y));
        }

        #fillStyle = '#000000';
        #strokeStyle = '#000000';

        get fillStyle() {
            return this.#fillStyle;
        }

        set fillStyle(color) {
            this.#fillStyle = QJSC_fillStyle(this.#id, String(color)) || this.fillStyle;
        }

        get strokeStyle() {
            return this.#strokeStyle;
        }

        set strokeStyle(color) {
            this.#strokeStyle = QJSC_strokeStyle(this.#id, String(color)) || this.strokeStyle;
        }

        #path = [];
        #currentPath;

        beginPath() {
            this.#path = [];
            this.currentPath = undefined;
        }

        moveTo(x, y) {
            const newPath = [[int(x), int(y)]];
            this.#currentPath = newPath;
            this.#path.push(newPath);
        }

        lineTo(x, y) {
            if (this.#currentPath) {
                this.#currentPath.push([int(x), int(y)]);
            } else {
                this.moveTo(x, y);
            }
        }

        closePath() {
            if (this.#currentPath) {
                this.#currentPath.push(this.#currentPath[0]);
                this.#currentPath = undefined;
            }
        }

        stroke() {
            QJSC_stroke(this.#id, this.#path);
            this.beginPath();
        }
        fill() {
            QJSC_fill(this.#id, this.#path);
            this.beginPath();
        }
    }

    class Window {
        #id;

        constructor(id) {
            this.#id = id;
        }

        #title = 'quickjs-canvas';

        get title() {
            return this.#title;
        }

        set title(prop) {
            this.#title = String(prop);
            QJSC_setTitle(this.#id, this.#title);
        }

        // close
        // moveTo
        // moveBy

    }

    const performance = {
        now: function () {
            return Date.now() - initialTime;
        },
    };

    const frameQueue = [];

    function requestAnimationFrame(fn) {
        frameQueue.push(fn);
    }

    function flushRAFQueue() {
        const processQueue = frameQueue.splice(0, frameQueue.length);
        while (processQueue.length > 0) {
            processQueue.pop()(performance.now());
        }
    }

    function alert(text) {
        QJSC_msgBox("alert", String(text || ''));
    }

    function confirm(text) {
        return QJSC_msgBox("confirm", String(text || ''));
    }

    Object.assign(globalThis, {
        // public
        Canvas: HTMLCanvasElement,
        requestAnimationFrame,
        performance,
        alert,
        confirm,
        assertEq: (a, b) => { if (a !== b) throw new Error(`${a} != ${b}`); },
        assert: (a) => { if (!a) throw new Error(a); },
        // private
        flushRAFQueue,
    });

    ['flushRAFQueue']
        .forEach(key => {
            Object.defineProperty(globalThis, key, {
                enumerable: false,
                configurable: false,
                writable: false,
            });
        });
})();
