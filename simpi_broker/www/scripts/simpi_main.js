/*!simpi.js
 * Main class for SimPi front end user interface.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

 class SimPi {
    constructor() {
        this.gpioregs = new GpioRegs(document.getElementById("gpioregs"));
        this.themeManager = new ThemeManager({
            darkModeStylesheetURL: "styles/simpi-dark.css",
            mode: THM.mode.auto,
        });
        this.prefs = new Preferences(document.getElementById("preferences"));
        this.prefs.addChangeListener("general__theme", (v) => {
            this.themeManager.set(v);
        });
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
        this.isPaused = true;
        this.periodicSyncId = null;
        this.ele = {
            LED1: document.getElementById("LED1"),
            LED2: document.getElementById("LED2"),
            LED3: document.getElementById("LED3"),
            LED4: document.getElementById("LED4"),
            BTN1: document.getElementById("BTN1"),
            BTN2: document.getElementById("BTN2"),
            BTN3: document.getElementById("BTN3"),
            ctrlSettings: document.getElementById("ctrl-settings"),
            ctrlPause: document.getElementById("ctrl-pause"),
            ctrlPlay: document.getElementById("ctrl-play"),
            ctrlReset: document.getElementById("ctrl-reset"),
            ctrlTerminate: document.getElementById("ctrl-terminate"),
            statusConnectivity: document.getElementById("status-connectivity"),
        };
        this.arrBTN.forEach((v, i) => {
            this.ele[v].addEventListener("mousedown", (e) => {
                e.target.dataset.value = "1";
                this.gpioregs.$.input.writePin(this.h2g[v], 1);
            });
            this.ele[v].addEventListener("mouseup", (e) => {
                e.target.dataset.value = "0";
                this.gpioregs.$.input.writePin(this.h2g[v], 0);
            });
        });
        this.ele.ctrlTerminate.addEventListener("click", () => {
            this.terminate();
        });
        this.ele.ctrlSettings.addEventListener("click", () => {
            this.prefs.prefElement.dataset.visible = true;
        });
        this.ele.ctrlReset.addEventListener("click", () => {
            this.reset();
        });
        this.ele.ctrlPause.addEventListener("click", () => {
            this.pause();
        });
        this.ele.ctrlPlay.addEventListener("click", () => {
            this.play();
        });
        this.prefs.init().then(() => {
            this.play();
        });
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
        this.periodicSyncId = setInterval(() => {
            this.syncData();
        }, this.prefs.get("sync__update_timeout_ms"));
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
                let parsedData = SimPi.parseSimPiTransferData(data);
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
        fetch("/api/action/terminate").then((response) => {
            response.text().then((data) => {
                let parsedData = SimPi.parseSimPiTransferData(data);
                if (parsedData[0].status == "SUCC") {
                    document.write("<span style='font-style:italic;font-family:sans-serif'>Terminated SimPi Broker. You can now close this browser tab.");
                }
            });
        }).catch((err) => {
            alert("SimPi Broker is either already terminated or the action failed to succeed.");
        });
    }
    
    /**
     * Syncs all registers.
     */
    syncData() {
        if (this.isPaused) { return; }
        let getURL = "/api/getreg/" +
            this.gpioregs.$.output.key + ";" +
            this.gpioregs.$.config.key + ";" +
            this.gpioregs.$.pwm.key + ";" +
            this.gpioregs.$.inten.key + ";" +
            this.gpioregs.$.int0.key + ";" +
            this.gpioregs.$.int1.key;
        fetch(getURL).then((response) => {
            response.text().then((data) => {
                let parsedData = SimPi.parseSimPiTransferData(data);
                parsedData.forEach((v, i) => {
                    this.gpioregs.$[v.key].fromString(v.value);
                    if (v.key == this.gpioregs.$.output.key) {
                        this.ele["LED1"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED1"]));
                        this.ele["LED2"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED2"]));
                        this.ele["LED3"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED3"]));
                        this.ele["LED4"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED4"]));
                    }
                });
            });
            this.ele.statusConnectivity.dataset.state = "on";
        }).catch((err) => {
            this.ele.statusConnectivity.dataset.state = "off";
        });
        let setURL = "/api/setreg/" +
            this.gpioregs.$.input.key + "=" + this.gpioregs.$.input.toString();
        fetch(setURL).catch((err) => {
            this.ele.statusConnectivity.dataset.state = "off";
        });
    }

    /**
     * Parses an raw response string into an JS object and returns it.
     * @param {String} data The raw string to be parsed.
     * @returns {Object}
     */
    static parseSimPiTransferData(data) {
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
