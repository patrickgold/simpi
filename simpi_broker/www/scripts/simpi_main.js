/*!simpi.js
 * Main class for SimPi front end user interface.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

 class SimPi {
    constructor () {
        this.gpioregs = new GpioRegs(document.getElementById("gpioregs"));
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
                this.setAttribute("data-value", "1");
                that.gpioregs.$.input.writePin(that.h2g[v], 1);
            });
            that.ele[v].addEventListener("mouseup", function (e) {
                this.setAttribute("data-value", "0");
                that.gpioregs.$.input.writePin(that.h2g[v], 0);
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
        this.ele.ctrlTerminate.addEventListener("click", () => { that.terminate(); });
        this.ele.ctrlReset.addEventListener("click", () => { that.reset(); });
        this.ele.ctrlPause.addEventListener("click", () => { that.pause(); });
        this.ele.ctrlPlay.addEventListener("click", () => { that.play(); });
    }

    /**
     * Pauses the SimPi Client.
     */
    pause() {
        if (this.isPaused) { return; }
        this.isPaused = true;
        this.ele.ctrlPause.classList.add("hide");
        this.ele.ctrlPlay.classList.remove("hide");
        clearInterval(this.periodicSyncId);
    }

    /**
     * Unpauses the SimPi Client.
     */
    play() {
        if (!this.isPaused) { return; }
        this.isPaused = false;
        this.ele.ctrlPlay.classList.add("hide");
        this.ele.ctrlPause.classList.remove("hide");
        this.periodicSyncId = setInterval(this.syncData, this.updateSpeedMS, this);
    }

    /**
     * Resets the whole SimPi Client and Broker (NOT the wiringPi side though!!!).
     */
    reset() {
        this.arrBTN.forEach((v, i) => {
            this.ele[v].dataset.value = "0";
        }, this);
        this.gpioregs.reset();
        this.gpioregs.$.input.syncAllToUi();
        let that = this;
        fetch("/api/action/reset").then((response) => {
            response.text().then((data) => {
                let parsedData = that.parseSimPiTransferData(data);
                if (parsedData[0].status == "SUCC") {
                    alert("Reset done on SimPi Broker.");
                }
            });
        }).catch((err) => {
            alert("Error: Couldn't reach SimPi Broker.");
            that.ele.statusConnectivity.dataset.state = "off";
        });
    }

    /**
     * Terminate the SimPi Broker (and then the SimPi Client) if possible.
     */
    terminate() {
        let that = this;
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
    }
    
    /**
     * Syncs all registers.
     * @param {SimPi} that Reference to the 'this' object of SimPi.
     */
    syncData(that) {
        if (that.isPaused) { return; }
        let getURL = "/api/getreg/" +
            that.gpioregs.$.output.key + ";" +
            that.gpioregs.$.config.key + ";" +
            that.gpioregs.$.pwm.key + ";" +
            that.gpioregs.$.inten.key + ";" +
            that.gpioregs.$.int0.key + ";" +
            that.gpioregs.$.int1.key;
        fetch(getURL).then((response) => {
            response.text().then((data) => {
                let parsedData = that.parseSimPiTransferData(data);
                parsedData.forEach((v, i) => {
                    that.gpioregs.$[v.key].fromString(v.value);
                    if (v.key == that.gpioregs.$.output.key) {
                        that.ele["LED1"].setAttribute("data-value", that.gpioregs.$.output.readPin(that.h2g["LED1"]));
                        that.ele["LED2"].setAttribute("data-value", that.gpioregs.$.output.readPin(that.h2g["LED2"]));
                        that.ele["LED3"].setAttribute("data-value", that.gpioregs.$.output.readPin(that.h2g["LED3"]));
                        that.ele["LED4"].setAttribute("data-value", that.gpioregs.$.output.readPin(that.h2g["LED4"]));
                    }
                });
            });
            that.ele.statusConnectivity.dataset.state = "on";
        }).catch((err) => {
            that.ele.statusConnectivity.dataset.state = "off";
        });
        let setURL = "/api/setreg/" +
            that.gpioregs.$.input.key + "=" + that.gpioregs.$.input.toString();
        fetch(setURL).catch((err) => {
            that.ele.statusConnectivity.dataset.state = "off";
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
