const SP = Object.freeze({
    LED1: "GPIO18",
    LED2: "GPIO23",
    LED3: "GPIO24",
    LED4: "GPIO25",
    BTN1: "GPI022",
    BTN2: "GPIO27",
    BTN3: "GPIO18",
});

class SimPi {
    constructor () {
        this.h2g = {
            LED1: "GPIO18",
            LED2: "GPIO23",
            LED3: "GPIO24",
            LED4: "GPIO25",
            BTN1: "GPIO22",
            BTN2: "GPIO27",
            BTN3: "GPIO17",
        };
        this.g2h = {
            GPIO18: "LED1",
            GPIO23: "LED2",
            GPIO24: "LED3",
            GPIO25: "LED4",
            GPIO22: "BTN1",
            GPIO27: "BTN2",
            GPIO17: "BTN3",
        };
        this.arrLED = [ "LED1", "LED2", "LED3", "LED4", ];
        this.arrBTN = [ "BTN1", "BTN2", "BTN3", ];
        this.buttonStates = { BTN1: 0, BTN2: 0, BTN3: 0, };
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
                that.buttonStates[v] = 1;
                this.setAttribute("data-value", "1");
            });
            that.ele[v].addEventListener("mouseup", function (e) {
                that.buttonStates[v] = 0;
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
                    if (parsedData[0].status == "success") {
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
                    if (parsedData[0].status == "success") {
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
    
    syncData(that) {
        let urlLED = "/api/getpin/";
        that.arrLED.forEach((v, i) => {
            urlLED += that.h2g[v] + ";";
        });
        urlLED.slice(0, -1);
        fetch(urlLED).then((response) => {
            response.text().then((data) => {
                let parsedData = that.parseSimPiTransferData(data);
                parsedData.forEach((v, i) => {
                    if (!that.isPaused) {
                        that.ele[that.g2h[v.pin_name]].setAttribute("data-value", v.value);
                    }
                });
            });
            that.ele.statusConnectivity.setAttribute("data-state", "on");
        }).catch((err) => {
            that.ele.statusConnectivity.setAttribute("data-state", "off");
        });
        let urlBTN = "/api/setpin/";
        that.arrBTN.forEach((v, i) => {
            urlBTN += that.h2g[v] + "=" + that.buttonStates[v] + ";";
        });
        urlBTN.slice(0, -1);
        fetch(urlBTN).catch((err) => {
            that.ele.statusConnectivity.setAttribute("data-state", "off");
        });
    }

    parseSimPiTransferData(data) {
        let ret = [];
        data.split("\n\n").forEach((v, i) => {
            let retObj = {};
            v.split("\n").forEach((v, i) => {
                let kv = v.split(":");
                retObj[kv[0]] = kv[1];
            });
            ret.push(retObj);
        });
        return ret;
    }
}
