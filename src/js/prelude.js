'use strict';

(function() {
    // https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement

    let id = 0;

    const float = (num) => num + Number.EPSILON;

    class HTMLCanvasElement {
        constructor() {
            this.id = id++;
            QJSC_initCanvas(this.id);
        }

        getContext(type) {
            if (type !== '2d') {
                throw new Error(`Currently only 2D rendering is supported`);
            }
            return new CanvasRenderingContext2D(this.id);
        };

        getWindow() {
            return new Window(this.id);
        }

        // #width = 300;
        // #height = 150;

        // get width() {
        //     return this.#width;
        // }

        // get height() {
        //     return this.#height;
        // }

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
    }

    class Window {
        constructor(id) {
            this.id = id;
        }
        // title
        // close
        // moveTo

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
