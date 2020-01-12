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
            ctrlAbout: document.getElementById("ctrl-about"),
            ctrlSettings: document.getElementById("ctrl-settings"),
            ctrlPause: document.getElementById("ctrl-pause"),
            ctrlPlay: document.getElementById("ctrl-play"),
            ctrlReset: document.getElementById("ctrl-reset"),
            ctrlTerminate: document.getElementById("ctrl-terminate"),
            statusBroker: document.getElementById("status-broker"),
            statusWpiSim: document.getElementById("status-wpisim"),
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
        this.ele.ctrlAbout.addEventListener("click", () => {
            alert("See https://github.com/patrickgold/simpi");
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
        this.setupWebSocket();
    }

    /**
     * Sets up the websocket connection.
     */
    setupWebSocket() {
        this.webSocket = new WebSocket("ws://127.0.0.1:32001", "rust-websocket");
        this.webSocket.onmessage = (ev) => this.receiveData(ev);
        this.webSocket.onclose = (ev) => {
            this.ele.statusWpiSim.dataset.state = "off";
        };
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
            this.ele.statusBroker.dataset.state = "off";
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
     * Receives data from the web socket and sets the pins accordingly.
     * @param {MessageEvent} event The event data of the web socket receiver.
     */
    receiveData(event) {
        if (this.isPaused) { return; }
        //console.log(event.data);
        this.ele.statusWpiSim.dataset.state = "on";
        let ret_data = SimPi.parseSimPiTransferData(event.data);
        if (ret_data.command.toLowerCase() == "getreg") {
            this.gpioregs.$[ret_data.key.toLowerCase()].fromString(ret_data.value);
            if (ret_data.key.toLowerCase() == "output") {
                this.ele["LED1"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED1"]));
                this.ele["LED2"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED2"]));
                this.ele["LED3"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED3"]));
                this.ele["LED4"].setAttribute("data-value", this.gpioregs.$.output.readPin(this.h2g["LED4"]));
            }
        }
    }
    
    /**
     * Syncs all registers.
     */
    syncData() {
        if (this.isPaused) { return; }
        if (this.webSocket.readyState == WebSocket.CLOSED) {
            this.setupWebSocket();
        }
        if (this.webSocket.readyState != WebSocket.OPEN) {
            return;
        }
        for (var v in this.gpioregs.$) {
            if (this.gpioregs.$.hasOwnProperty(v)) {
                let req_data = {
                    command: v == "input" ? "setreg" : "getreg",
                    key: this.gpioregs.$[v].key,
                    value: this.gpioregs.$[v].toString(),
                };
                this.webSocket.send(SimPi.packSimPiTransferData(req_data));
            }
        }
    }

    /**
     * Parses an raw response string into an JS object and returns it.
     * @param {String} data The raw string to be parsed.
     * @returns {Object}
     */
    static parseSimPiTransferData(data) {
        let ret = {};
        data = data.slice(1);
        data.split("/").forEach((v, i) => {
            if (i == 0) {
                v.split(":").forEach((vv, ii) => {
                    if (ii == 0) {
                        ret.command = vv;
                    } else if (ii == 1) {
                        ret.status = vv;
                    } else {
                        throw new Error("Invalid request syntax!");
                    }
                });
            } else if (i == 1) {
                v.split("=").forEach((vv, ii) => {
                    if (ii == 0) {
                        ret.key = vv;
                    } else if (ii == 1) {
                        ret.value = vv;
                    } else {
                        throw new Error("Invalid request syntax!");
                    }
                });
            } else {
                throw new Error("Invalid request syntax!");
            }
        });
        return ret;
    }

    /**
     * Packs an given object to a request string.
     * @param {Object} data The object to be packed.
     * @returns {String}
     */
    static packSimPiTransferData(data) {
        let ret = "";
        ret += data.command;
        ret += "/";
        ret += data.key;
        if (data.hasOwnProperty("value")) {
            ret += "=";
            ret += data.value;
        }
        return ret;
    }
}
