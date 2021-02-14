'use strict';

(function() {
    // https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement

    let id = 0;

    // const float = (num) => num + Number.EPSILON;
    const int = (num) => Math.round(num);

    class HTMLCanvasElement {
        constructor(width = 300, height = 150) {
            this.id = id++;
            this.#width = int(width);
            this.#height = int(height);
            QJSC_initCanvas(this.id, this.#width, this.#height);
        }

        getContext(type) {
            if (type !== '2d') {
                throw new Error(`Currently only 2D rendering is supported`);
            }
            return new CanvasRenderingContext2D(this.id);
        };

        #window;

        get window() {
            if (!this.#window) {
                this.#window = new Window(this.id);
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
                QJSC_setSize(this.id, this.#width, this.#height);
            }
        }

        set height(prop) {
            const height = int(prop)
            if (height >= 0) {
                this.#height = height;
                QJSC_setSize(this.id, this.#width, this.#height);
            }
        }

        // style {backgroundColor, cursor}

        //toDataURL
        //toBlob
    }

    // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D

    class CanvasRenderingContext2D {
        constructor(id) {
            this.id = id;
        }

        clearRect(x, y, w, h) {
            QJSC_clearRect(this.id, int(x), int(y), int(w), int(h));
        }
        fillRect(x, y, w, h) {
            QJSC_fillRect(this.id,  int(x), int(y), int(w), int(h));
        }
        strokeRect(x, y, w, h) {
            QJSC_strokeRect(this.id,  int(x), int(y), int(w), int(h));
        }

        fillText(text, x, y) {
            QJSC_fillText(this.id, String(text), int(x), int(y));
        }

        #fillStyle = '#000000';
        #strokeStyle = '#000000';

        get fillStyle() {
            return this.#fillStyle;
        }

        set fillStyle(color) {
            this.#fillStyle = QJSC_fillStyle(this.id, String(color)) || this.fillStyle;
        }

        get strokeStyle() {
            return this.#strokeStyle;
        }

        set strokeStyle(color) {
            this.#strokeStyle = QJSC_strokeStyle(this.id, String(color)) || this.fillStyle;
        }
    }

    class Window {
        constructor(id) {
            this.id = id;
        }

        #title = 'quickjs-canvas';

        get title() {
            return this.#title;
        }

        set title(prop) {
            this.#title = String(prop);
            QJSC_setTitle(this.id, this.#title);
        }

        // close
        // moveTo
        // moveBy

    }

    // alert() global

    const frameQueue = [];

    function requestAnimationFrame(fn) {
        frameQueue.push(fn);
    }

    function flushRAFQueue() {
        const processQueue = frameQueue.splice(0, frameQueue.length);
        while (processQueue.length > 0) {
            processQueue.pop()();
        }
    }

    Object.assign(globalThis, {
        // public
        Canvas: HTMLCanvasElement,
        requestAnimationFrame,
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
