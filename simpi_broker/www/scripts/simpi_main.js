/*!simpi.js
 * Main class for SimPi front end user interface.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

 class SimPi {
    constructor () {
        this.gpioregs = new GpioRegs();
        this.h2g = {
            LED1: 18,
            LED2: 23,
            LED3: 24,
            LED4: 25,
            BTN1: 22,
            BTN2: 27,
            BTN3: 17,
        };
        this.g2h = {
            18: "LED1",
            23: "LED2",
            24: "LED3",
            25: "LED4",
            22: "BTN1",
            27: "BTN2",
            17: "BTN3",
        };
        this.arrLED = [ "LED1", "LED2", "LED3", "LED4", ];
        this.arrBTN = [ "BTN1", "BTN2", "BTN3", ];
        this.isPaused = false;
        this.updateSpeedMS = 100;
        this.periodicSyncId = null;
        this.ele = {
            LED1: document.getElementById("LED1"),
            LED2: document.getElementById("LED2"),
            LED3: document.getElementById("LED3"),
            LED4: document.getElementById("LED4"),
            BTN1: document.getElementById("BTN1"),
            BTN2: document.getElementById("BTN2"),
            BTN3: document.getElementById("BTN3"),
            prefUpdateSpeed_i: document.getElementById("pref-update-speed-i"),
            prefUpdateSpeed_o: document.getElementById("pref-update-speed-o"),
            ctrlSettings: document.getElementById("ctrl-settings"),
            ctrlPause: document.getElementById("ctrl-pause"),
            ctrlPlay: document.getElementById("ctrl-play"),
            ctrlReset: document.getElementById("ctrl-reset"),
            ctrlTerminate: document.getElementById("ctrl-terminate"),
            statusConnectivity: document.getElementById("status-connectivity"),
        };
        let that = this;
        this.arrBTN.forEach((v, i) => {
            that.ele[v].addEventListener("mousedown", function (e) {
                that.gpioregs.writePin(that.h2g[v], 1, GPIOREGS.input);
                this.setAttribute("data-value", "1");
            });
            that.ele[v].addEventListener("mouseup", function (e) {
                that.gpioregs.writePin(that.h2g[v], 0, GPIOREGS.input);
                this.setAttribute("data-value", "0");
            });
        });
        this.ele.prefUpdateSpeed_i.addEventListener("input", function (e) {
            that.ele.prefUpdateSpeed_o.innerHTML = this.value + " ms";
            that.updateSpeedMS = this.value;
            clearInterval(that.periodicSyncId);
            that.periodicSyncId = setInterval(that.syncData, that.updateSpeedMS, that);
        });
        this.ele.prefUpdateSpeed_i.addEventListener("change", function (e) {
            that.ele.prefUpdateSpeed_o.innerHTML = this.value + " ms";
        });
        this.ele.prefUpdateSpeed_i.dispatchEvent(new Event('input', {
            bubbles: true,
            cancelable: true,
        }));
        this.ele.ctrlTerminate.addEventListener("click", function (e) {
            fetch("/api/action/terminate").then((response) => {
                response.text().then((data) => {
                    let parsedData = that.parseSimPiTransferData(data);
                    if (parsedData[0].status == "SUCC") {
                        document.write("Terminated SimPi Broker. You can now close this browser tab.");
                    }
                });
            }).catch((err) => {
                alert("SimPi Broker is either already terminated or the action failed to succeed.");
            });
        });
        this.ele.ctrlReset.addEventListener("click", function (e) {
            fetch("/api/action/reset").then((response) => {
                response.text().then((data) => {
                    let parsedData = that.parseSimPiTransferData(data);
                    if (parsedData[0].status == "SUCC") {
                        alert("Reset done on SimPi Broker.");
                    }
                });
            }).catch((err) => {
                alert("Error: Couldn't reach SimPi Broker.");
            });
        });
        this.ele.ctrlPause.addEventListener("click", function (e) {
            that.isPaused = true;
            this.classList.add("hide");
            that.ele.ctrlPlay.classList.remove("hide");
            clearInterval(that.periodicSyncId);
        });
        this.ele.ctrlPlay.addEventListener("click", function (e) {
            that.isPaused = false;
            this.classList.add("hide");
            that.ele.ctrlPause.classList.remove("hide");
            that.periodicSyncId = setInterval(that.syncData, 100, that);
        });
    }
    
    /**
     * Syncs all registers.
     * @param {SimPi} that Reference to the 'this' object of SimPi.
     */
    syncData(that) {
        if (that.isPaused) {
            return;
        }
        let getURL = "/api/getreg/" +
            GPIOREGS.output + ";" +
            GPIOREGS.config + ";" +
            GPIOREGS.pwm + ";" +
            GPIOREGS.intrp;
        fetch(getURL).then((response) => {
            response.text().then((data) => {
                let parsedData = that.parseSimPiTransferData(data);
                parsedData.forEach((v, i) => {
                    that.gpioregs[v.key] = that.gpioregs.strToReg(v.value);
                    if (v.key == GPIOREGS.output) {
                        that.ele["LED1"].setAttribute("data-value", that.gpioregs.readPin(that.h2g["LED1"], GPIOREGS.output));
                        that.ele["LED2"].setAttribute("data-value", that.gpioregs.readPin(that.h2g["LED2"], GPIOREGS.output));
                        that.ele["LED3"].setAttribute("data-value", that.gpioregs.readPin(that.h2g["LED3"], GPIOREGS.output));
                        that.ele["LED4"].setAttribute("data-value", that.gpioregs.readPin(that.h2g["LED4"], GPIOREGS.output));
                    }
                });
            });
            that.ele.statusConnectivity.setAttribute("data-state", "on");
        }).catch((err) => {
            that.ele.statusConnectivity.setAttribute("data-state", "off");
        });
        let setURL = "/api/setreg/" +
            GPIOREGS.input + "=" + that.gpioregs.regToStr(GPIOREGS.input);
        fetch(setURL).catch((err) => {
            that.ele.statusConnectivity.setAttribute("data-state", "off");
        });
    }

    /**
     * Parses an raw response string into an JS object and returns it.
     * @param {String} data The raw string to be parsed.
     * @returns {Object}
     */
    parseSimPiTransferData(data) {
        let ret = [];
        data.split("\n").forEach((v, i) => {
            if (v.startsWith(">")) {
                v = v.slice(1);
                let retObj = {};
                v.split(";").forEach((v, i) => {
                    if (i == 0) {
                        retObj["status"] = v;
                    } else if (i == 1) {
                        retObj["key"] = v;
                    } else if (i == 2) {
                        retObj["value"] = v;
                    }
                });
                ret.push(retObj);
            }
        });
        return ret;
    }
}
