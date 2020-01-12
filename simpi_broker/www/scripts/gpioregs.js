/*!gpioregs.js
 * Class for working with the simulated Raspberry Pi GPIO Registers.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

class GpioReg {
    "use strict";

    /**
     * Initializes a new GPIO Register.
     * @param {String} key The key name of the register.
     * @param {number} defaultValue The default value used on init and reset.
     */
    constructor(key, defaultValue, minNum = 0, maxNum = 31) {
        this.key = key;
        this._defaultValue = (defaultValue >>> 0);
        this._reg = (defaultValue >>> 0);
        this.minNum = minNum;
        this.maxNum = maxNum;
        this.uiBits = [];
        this.uiName = null;
    }

    /**
     * Resets the register to default value.
     */
    reset() {
        this._reg = this._defaultValue;
    }

    /**
     * Read the register.
     * @returns {number} (unsigned)
     */
    read() {
        return (this._reg >>> 0);
    }
    /**
     * Writes a given value to the register.
     * @param {number} v The new value of the register.
     */
    write(v) {
        this._reg = (v >>> 0);
        this.syncAllToUi();
    }

    /**
     * Reads the register and returns it as string.
     * @returns {String}
     */
    toString() {
        return "0x" + this._reg.toString(16);
    }
    /**
     * Writes a given value (as string) to the register.
     * @param {String} v The new value of the register.
     */
    fromString(v) {
        this._reg = (parseInt(v, 16) >>> 0);
        this.syncAllToUi();
    }

    /**
     * Reads a given pin number and returns its value.
     * @param {number} pin The pin number to be read.
     * @returns {number}
     */
    readPin(pin) {
        return (this._reg >>> pin) & 0x1;
    }
    /**
     * Writes a given value to the given pin.
     * @param {number} pin The pin number to be written.
     * @param {number} v The value to be written.
     */
    writePin(pin, v) {
        if (v == 0) {
            this._reg = ((this._reg & ~(0x1 << pin)) >>> 0);
        } else {
            this._reg = ((this._reg | (0x1 << pin)) >>> 0);
        }
        this.syncSingleToUi(pin);
    }

    /**
     * Syncs the value of a given pin to UI.
     * @param {number} pin The pin to be synced.
     */
    syncSingleToUi(pin) {
        if (this.uiBits.length > pin) {
            this.uiBits[pin].dataset.value = this.readPin(pin);
        }
    }
    /**
     * Syncs the whole register to UI.
     */
    syncAllToUi() {
        for (let b = 0; b < this.uiBits.length; b++) {
            this.uiBits[b].dataset.value = this.readPin(b);
        }
    }
}

class GpioRegs {
    "use strict";

    /**
     * Initializes a new GPIO Registers object.
     * @param {HTMLElement} rootRegEle The root element to build the register matrix on.
     */
    constructor(rootRegEle) {
        this.$ = {
            input:      new GpioReg("input",    0x00000000, 2, 27),
            output:     new GpioReg("output",   0x00000000, 2, 27),
            config:     new GpioReg("config",   0xFFFFFFFF, 2, 27),
            inten:      new GpioReg("inten",    0x00000000, 2, 27),
            int0:       new GpioReg("int0",     0x00000000, 2, 27),
            int1:       new GpioReg("int1",     0x00000000, 2, 27),
        };
        this.reset();
        this.setupRegUI(rootRegEle);
    }

    /**
     * Sets up the UI for the registers.
     * @param {HTMLElement} rootRegEle The root register element to build the UI.
     */
    setupRegUI(rootRegEle) {
        let regArray = ["header"].concat(Object.keys(this.$));

        regArray.forEach((v, i) => {
            let reg = document.createElement("div");
            reg.id = "reg_" + v;
            reg.classList.add("reg");
            if (v == "header") {
                reg.classList.add("rheader");
            }

            let regName = document.createElement("div");
            regName.classList.add("name");
            if (v != "header") {
                regName.innerHTML = v.toUpperCase();
                this.$[v].uiName = regName;
            } else {
                regName.innerHTML = "GPIO";
            }
            reg.appendChild(regName);

            let regBits = document.createElement("div");
            regBits.classList.add("bits");
            reg.appendChild(regBits);

            for (let b = 31; b >= 0; b--) {
                let regBit = document.createElement("button");
                regBit.id = "reg_" + v + "_" + b;
                regBit.classList.add("bit");
                if (b <= this._max_num && b >= this._min_num) {
                    if (v == "header") {
                        regBit.dataset.type = "hv";
                        regBit.setAttribute("disabled", "");
                    } else {
                        regBit.dataset.type = "v";
                        regBit.dataset.value = "0";
                    }
                    
                } else {
                    if (v == "header") {
                        regBit.dataset.type = "hu";
                    } else {
                        regBit.dataset.type = "u";
                    }
                    regBit.dataset.value = "0";
                    regBit.setAttribute("disabled", "");
                }
                if (v == "header") {
                    regBit.innerHTML = b;
                } else {
                    this.$[v].uiBits[b] = regBit;
                }
                regBits.appendChild(regBit);
            }

            rootRegEle.appendChild(reg);
        }, this);
    }

    /**
     * Resets all registers to default value.
     */
    reset() {
        for (let key in this.$) {
            if (this.$.hasOwnProperty(key)) {
                this.$[key].reset();
            }
        }
        this._min_num = 2;
        this._max_num = 27;
    }
}
