/*
 *
 * TODO
 *
 * add object-inspect
 */

(function() {
    let id = 0;

    class HTMLCanvasElement {
        constructor() {
            this.id = id++;
        }

        getContext = () => new Context(this.id);

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

    class Context {

    }

    Object.assign(globalThis, {
        Canvas: HTMLCanvasElement,
    });
})();
