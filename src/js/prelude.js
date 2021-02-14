'use strict';

(function() {
    // https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement

    let id = 0;

    const float = (num) => num + Number.EPSILON;
    const int = (num) => Math.floor(num);

    class HTMLCanvasElement {
        constructor(width = 300, height = 150) {
            this.id = id++;
            QJSC_initCanvas(this.id, int(width), int(height));
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

        #width = 300;
        #height = 150;

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

        fillText(text, x, y) {
            QJSC_fillText(this.id, String(text), float(x), float(y));
        }
        clearRect(x, y, w, h) {
            QJSC_clearRect(this.id, float(x), float(y), float(w), float(h));
        }

        // clear [new]
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
