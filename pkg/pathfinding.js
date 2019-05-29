import * as wasm from './pathfinding_bg';

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* @param {any} grid
* @param {any} start
* @param {any} end
* @param {any} opts
* @returns {any}
*/
export function findPath(grid, start, end, opts) {
    try {
        return takeObject(wasm.findPath(addBorrowedObject(grid), addBorrowedObject(start), addBorrowedObject(end), addBorrowedObject(opts)));

    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;

    }

}

/**
* @param {any} grid
* @param {any} source
* @param {any} opts
* @returns {any}
*/
export function findWalkable(grid, source, opts) {
    try {
        return takeObject(wasm.findWalkable(addBorrowedObject(grid), addBorrowedObject(source), addBorrowedObject(opts)));

    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;

    }

}

/**
* @param {any} coords
* @returns {any}
*/
export function toCoordMap(coords) {
    try {
        return takeObject(wasm.toCoordMap(addBorrowedObject(coords)));

    } finally {
        heap[stack_pointer++] = undefined;

    }

}

/**
*/
export const GridType = Object.freeze({ Cardinal:0,Hex:1,Intercardinal:2, });

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

export function __wbindgen_json_parse(ptr, len) { return addHeapObject(JSON.parse(getStringFromWasm(ptr, len))); }

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

export function __wbindgen_json_serialize(idx, ptrptr) {
    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
    getUint32Memory()[ptrptr / 4] = ptr;
    return WASM_VECTOR_LEN;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

function freeCoord(ptr) {

    wasm.__wbg_coord_free(ptr);
}
/**
*/
export class Coord {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeCoord(ptr);
    }

    /**
    * @returns {number}
    */
    get x() {
        return wasm.__wbg_get_coord_x(this.ptr);
    }
    set x(arg0) {
        return wasm.__wbg_set_coord_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        return wasm.__wbg_get_coord_y(this.ptr);
    }
    set y(arg0) {
        return wasm.__wbg_set_coord_y(this.ptr, arg0);
    }
}

function freeGrid(ptr) {

    wasm.__wbg_grid_free(ptr);
}
/**
*/
export class Grid {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeGrid(ptr);
    }

}

function freeSearchOpts(ptr) {

    wasm.__wbg_searchopts_free(ptr);
}
/**
*/
export class SearchOpts {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeSearchOpts(ptr);
    }

    /**
    * @returns {number | undefined}
    */
    get cost_threshold() {
        const retptr = globalArgumentPtr();

        wasm.__wbg_get_searchopts_cost_threshold(retptr, this.ptr);
        const present = getUint32Memory()[retptr / 4];
        const value = getInt32Memory()[retptr / 4 + 1];
        return present === 0 ? undefined : value;

    }
    set cost_threshold(arg0) {
        return wasm.__wbg_set_searchopts_cost_threshold(this.ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {boolean | undefined}
    */
    get end_on_unstoppable() {

        const ret = wasm.__wbg_get_searchopts_end_on_unstoppable(this.ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;

    }
    set end_on_unstoppable(arg0) {
        return wasm.__wbg_set_searchopts_end_on_unstoppable(this.ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
}

