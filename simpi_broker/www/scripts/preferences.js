/*!preferences.js
 * Class for working with the preferences.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

class Preferences {
    "use strict";

    /**
     * Constructs and initializes a new instance of Preferences.
     * @param {HTMLElement} prefEle A reference to the preferences DOM Element.
     */
    constructor(prefEle) {
        this.__defaultPrefs = {};
        this.__localPrefs = {};
        this.prefElement = prefEle;
        this.__callbacks = {};
    }

    async init() {
        await this.initUiAndPrefs("/data/prefs_config.json");
        await this.syncBrokerToLocal();
        this.syncLocalToUi();
    }

    /**
     * Reads a preference from the local preference storage. Returns the value
     * associated with the pref or null, if no pref is associated with the
     * provided key.
     * @param {string} key The key of the preference to be read.
     */
    get(key) {
        if (this.__localPrefs.hasOwnProperty(key)) {
            return this.__localPrefs[key];
        } else {
            return null;
        }
    }

    /**
     * Writes a preference to the local preference storage. Returns nothing.
     * Note: to save the data to the server, use syncLocalToBroker() !
     * @param {string} key The key of the preference to be written.
     * @param {any} value The value of the preference to be written.
     */
    set(key, value) {
        this.__localPrefs[key] = value;
        if (this.__callbacks.hasOwnProperty(key)) {
            this.__callbacks[key](value);
        }
    }

    /**
     * Reset settings to default.
     */
    reset() {
        this.__localPrefs = JSON.parse(JSON.stringify(this.__defaultPrefs))
    }

    async syncBrokerToLocal() {
        await fetch("/api/prefs").then(async (response) => {
            await response.json().then((prefs) => {
                this.__localPrefs = prefs;
                for (let key in this.__callbacks) {
                    if (this.__callbacks.hasOwnProperty(key)) {
                        this.__callbacks[key](this.__localPrefs[key]);
                    }
                }
                this.isReady = true;
                document.getElementById("status-broker").dataset.state = "on";
                console.info("Successfully synced prefs from broker.");
            }).catch((err) => {
                console.error(err);
            });
        }).catch((err) => {
            console.error(err);
        });
    }

    async syncLocalToBroker() {
        await fetch("/api/prefs", {
            method: "PUT",
            headers: { 'Content-Type': 'application/json' },
            referrerPolicy: "no-referrer",
            body: JSON.stringify(this.__localPrefs)
        }).then(async (response) => {
            await response.text().then((status) => {
                if (status != "SUCC") {
                    console.error("Failed to sync prefs to broker.");
                } else {
                    console.info("Successfully synced prefs to broker.");
                }
            }).catch((err) => {
                console.error(err);
            });
        }).catch((err) => {
            console.error(err);
        });
    }

    /**
     * Syncs local storage to UI.
     */
    async syncLocalToUi() {
        for (let key in this.__defaultPrefs) {
            if (this.__defaultPrefs.hasOwnProperty(key)) {
                let pref = document.getElementById(key);
                pref.value = this.get(key);
                if (pref.tagName.toLowerCase() == "input") {
                    if (pref.type == "range") {
                        pref.dispatchEvent(new Event("input"));
                    }
                }
            }
        }
    }

    /**
     * Syncs UI to local storage.
     */
    async syncUiToLocal() {
        for (let key in this.__defaultPrefs) {
            if (this.__defaultPrefs.hasOwnProperty(key)) {
                let pref = document.getElementById(key);
                this.set(key, pref.value);
            }
        }
    }

    /**
     * Initializes the UI and the preferences storage.
     * @param {String} prefConfigUrl A string containing the URL to the
     * prefs_config.json.
     */
    async initUiAndPrefs(prefConfigUrl) {
        await fetch(prefConfigUrl).then(async (response) => {
            await response.json().then((config) => {
                while (this.prefElement.firstChild) {
                    this.prefElement.removeChild(this.prefElement.firstChild);
                }
                this.prefElement.dataset.visible = false;
                let prefWindowEle = document.createElement("div");
                prefWindowEle.id = "preferences-window";
                this.prefElement.appendChild(prefWindowEle);
                let process = (prefGroupEle, chunk, intendLevel = 0) => {
                    if (chunk.hasOwnProperty("type")) {
                        if (chunk["type"].startsWith("group")) {
                            let prefGroupEleTmp = document.createElement("div");
                            prefGroupEleTmp.classList.add("pref-group");
                            prefGroupEleTmp.classList.add("lv" + intendLevel);
                            let h = document.createElement("h" + (intendLevel + 2));
                            h.innerHTML = chunk["heading"];
                            prefGroupEleTmp.appendChild(h);
                            if (chunk.hasOwnProperty("children")) {
                                if (Array.isArray(chunk["children"])) {
                                    chunk["children"].forEach((v, i) => {
                                        process(prefGroupEleTmp, v, intendLevel + 1);
                                    });
                                } else {
                                    console.error("Property 'children' must be an array!!");
                                }
                            }
                            prefGroupEle.appendChild(prefGroupEleTmp);
                        } else if (intendLevel == 0) {
                            console.error("Root level of pref config must be group!!");
                        } else if (chunk["type"].startsWith("pref")) {
                            this.__defaultPrefs[chunk["key"]] = chunk["defaultValue"];
                            let prefType = chunk["type"].split("/")[1];
                            let prefRow = document.createElement("div");
                            prefRow.classList.add("pref-row");
                            let label = document.createElement("label");
                            //label.htmlFor = chunk["key"];
                            label.innerHTML = chunk["label"];
                            prefRow.appendChild(label);
                            let input = document.createElement("input");
                            let unit = document.createElement("span");
                            unit.classList.add("unit");
                            if (prefType == "text") {
                                input.type = "text";
                                input.id = chunk["key"];
                                input.spellcheck = false;
                                input.value = chunk["defaultValue"];
                            } else if (prefType == "slider") {
                                input.type = "range";
                                input.id = chunk["key"];
                                input.value = chunk["defaultValue"];
                                input.min = chunk["min"];
                                input.max = chunk["max"];
                                input.step = chunk["step"];
                                input.addEventListener("input", (e) => {
                                    unit.innerHTML = e.target.value + " " +
                                        chunk["unit"];
                                });
                                unit.innerHTML = chunk["defaultValue"] + " " +
                                    chunk["unit"];
                            } else if (prefType == "dropdown") {
                                input = document.createElement("div");
                                input.classList.add("select");
                                let inputInner = document.createElement("select");
                                inputInner.id = chunk["key"];
                                chunk["options"].forEach((v, i) => {
                                    let option = document.createElement("option");
                                    option.innerHTML = v;
                                    if (v == chunk["defaultValue"]) {
                                        option.selected = "selected";
                                    }
                                    inputInner.appendChild(option);
                                });
                                input.appendChild(inputInner);
                            }
                            prefRow.appendChild(input);
                            if (unit.innerHTML != "") {
                                prefRow.appendChild(unit);
                            }
                            prefGroupEle.appendChild(prefRow);
                        }
                    } else {
                        console.error("Required field 'type' missing.");
                    }
                };
                process(prefWindowEle, config);
                this.reset(); // set local prefs to default for the beginning
                let buttonRow = document.createElement("div");
                buttonRow.classList.add("btn-row");
                let saveBtn = document.createElement("button");
                saveBtn.classList.add("green");
                saveBtn.innerHTML = "Save";
                saveBtn.addEventListener("click", (e) => {
                    this.syncUiToLocal();
                    this.syncLocalToBroker().then(() => {
                        this.prefElement.dataset.visible = false;
                    });
                });
                buttonRow.appendChild(saveBtn);
                let cancelBtn = document.createElement("button");
                cancelBtn.innerHTML = "Cancel";
                cancelBtn.addEventListener("click", (e) => {
                    this.syncLocalToUi();
                    this.prefElement.dataset.visible = false;
                });
                buttonRow.appendChild(cancelBtn);
                prefWindowEle.appendChild(buttonRow);
            }).catch((err) => {
                console.error(err);
            });
        }).catch((err) => {
            console.error(err);
        });
    }

    addChangeListener(key, callback) {
        this.__callbacks[key] = callback;
    }
    removeChangeListener(key) {
        delete this.__callbacks[key];
    }
}
