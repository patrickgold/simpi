/*!preferences.js
 * Class for working with the preferences.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

const PREFITEM = Object.freeze({
    type: Object.freeze({
        
    }),
    unit: Object.freeze({
        ms: "ms",
    })
});

class PreferenceItem {
    "use strict";

    constructor(name, type) {}
}

class Preferences {
    "use strict";

    constructor() {
        this.ui = {
            prefsWrapper: document.getElementById("preferences-wrapper"),
            prefs: document.getElementById("preferences"),
            updateSpeedIn: document.getElementById("pref-update-speed-i"),
            updateSpeedOut: document.getElementById("pref-update-speed-o"),
            hostnameIn: document.getElementById("pref-hostname-i"),
        };
        this.$ = {
            updateSpeedMs: 100,
            hostname: "",
        };
    }
}
