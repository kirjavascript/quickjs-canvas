/*
 *
 * TODO
 *
 * add object-inspect
 * title / get_window
 */

(function() {
    let id = 0;

    // https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement

    class HTMLCanvasElement {
        constructor() {
            this.id = id++;
        }

        getContext = (type) => {
            if (type !== '2d') {
                throw new Error(`Currently only 2D rendering is supported`);
            }
            return new CanvasRenderingContext2D(this.id);
        };

        #width = 300;
        #height = 150;

        get width() {
            return this.#width;
        }

        get height() {
            return this.#height;
        }

        //toDataURL
        //toBlob
    }

    // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D

    class CanvasRenderingContext2D {
    }

    Object.assign(globalThis, {
        Canvas: HTMLCanvasElement,
    });
})();
