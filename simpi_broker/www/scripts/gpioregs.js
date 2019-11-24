/*!gpioregs.js
 * Class for working with the simulated Raspberry Pi GPIO Registers.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

const GPIOREGS = Object.freeze({
    input: "input",
    output: "output",
    config: "config",
    pwm: "pwm",
    intrp: "intrp",
});

class GpioRegs {
    "use strict";

    constructor() {
        this.__valid_registers = [
            GPIOREGS.input,
            GPIOREGS.output,
            GPIOREGS.config,
            GPIOREGS.pwm,
            GPIOREGS.intrp,
        ];
        this.reset();
    }

    reset() {
        /** Input Register */
        this.input =    0x00000000;
        /** Output Register */
        this.output =   0x00000000;
        /** Config Register */
        this.config =   0xFFFFFFFF;
        /** PWM Register */
        this.pwm =      0x00000000;
        /** Interrupt Register */
        this.intrp =    0x00000000;

        this._min_num = 2;
        this._max_num = 27;
    }

    /**
     * Reads a specific {pin} in {reg} and returns its state. Throws an
     *  RangeError if specified {pin} and/or {reg} is out of range.
     * @param {number} pin The pin number to be read.
     * @param {String} reg The register.
     * @returns {number}
     */
    readPin(pin, reg) {
        if (this.__valid_registers.includes(reg)) {
            if (pin >= this._min_num && pin <= this._max_num) {
                return (this[reg] >>> pin) & 0x1;
            } else {
                throw new RangeError(
                    "readPin(): Specified pin '" + pin + "' is out of range."
                );
            }
        } else {
            throw new RangeError(
                "readPin(): Specified reg '" + reg + "' is not valid."
            );
        }
    }

    /**
     * Writes a specific {pin} in {reg}. Throws an RangeError if specified {pin}
     *  and/or {reg} is out of range.
     * @param {number} pin The pin number to be read.
     * @param {number} val The value.
     * @param {String} reg The register.
     */
    writePin(pin, val, reg) {
        if (this.__valid_registers.includes(reg)) {
            if (pin >= this._min_num && pin <= this._max_num) {
                if (val > 0) {
                    this[reg] |= (0x1 << pin);
                } else {
                    this[reg] &= ~(0x1 << pin);
                }console.log(this.input.toString(16));
            } else {
                throw new RangeError(
                    "writePin(): Specified pin '" + pin + "' is out of range."
                );
            }
        } else {
            throw new RangeError(
                "writePin(): Specified reg '" + reg + "' is not valid."
            );
        }
    }

    /**
     * Converts a register value to a string and returns it. Throws an
     *  RangeError if specified {reg} is not valid.
     * @param {String} reg The register name.
     * @returns {String}
     */
    regToStr(reg) {
        if (this.__valid_registers.includes(reg)) {
            return "0x" + this[reg].toString(16).toUpperCase();
        } else {
            throw new RangeError(
                "regToStr(): Specified reg '" + reg + "' is not valid."
            );
        }
    }

    /**
     * Converts a register string to a value and returns it.
     * @param {String} str The register string.
     * @returns {number}
     */
    strToReg(str) {
        return parseInt(str, 16);
    }
}
