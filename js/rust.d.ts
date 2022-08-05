/* tslint:disable */
/* eslint-disable */
/**
* @param {number} weight
* @param {number} threshold
* @param {number} five_min
* @param {number} one_min
* @param {number} five_sec
* @param {boolean} gender
* @returns {string}
*/
export function calculate_power(weight: number, threshold: number, five_min: number, one_min: number, five_sec: number, gender: boolean): string;
/**
* @param {number} hr_max
* @param {number} hr_rest
* @returns {string}
*/
export function calculate_hr_zones(hr_max: number, hr_rest: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_power: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_hr_zones: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
