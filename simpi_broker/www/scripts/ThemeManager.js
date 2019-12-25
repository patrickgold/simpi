/*!ThemeManager.js
 * Class file for the theme manager.
 * 
 * Author: Patrick Goldinger
 * License: GPL 3.0 (see LICENSE file for details)
 */

const THM = Object.freeze({
    mode: Object.freeze({
        auto: "auto",
        light: "light",
        dark: "dark",
    }),
});

class ThemeManager {
    "use strict";

    /**
     * Contructs a new instance of ThemeManager.
     * @param {Object} options Initial options.
     */
    constructor (options) {
        this._consoleLog("Setting up DarkModeToggle");
        this.options = Object.assign({
            darkModeStylesheet: null,
            darkModeStylesheetURL: "",
            mode: THM.mode.auto,
        }, options);
        this.set(this.options.mode);
    }

    _autoDetectMode() {
        if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
            return THM.mode.dark;
        } else {
            return THM.mode.light;
        }
    }

    _setTheme(mode) {
        let link = this.options.darkModeStylesheet;
        if (mode == THM.mode.light) {
            if (link != null) {
                link.parentElement.removeChild(link);
                this.options.darkModeStylesheet = null;
            }
        } else if (mode == THM.mode.dark) {
            if (link == null) {
                link = document.createElement("link");
                link.href = this.options.darkModeStylesheetURL;
                link.rel = "stylesheet";
                link.type = "text/css";
                this.options.darkModeStylesheet = link;
                document.head.appendChild(link);
            }
        }
    }

    /**
     * Sets the current theme.
     * @param {string} mode The name of the theme mode to be applied.
     */
    set(mode) {
        this.options.mode = mode;
        if (this.options.mode == THM.mode.auto) {
            this._setTheme(this._autoDetectMode());
        } else if (this.options.mode == THM.mode.light) {
            this._setTheme(THM.mode.light);
        } else if (this.options.mode == THM.mode.dark) {
            this._setTheme(THM.mode.dark);
        }
        this._consoleLog("Theme mode set: " + mode);
    }

    /**
     * Formats a string before outputting it.
     * @param {string} str The string to be formatted and outputted.
     */
    _consoleLog(str) {
        let prefix = ">> DarkModeToggle : ";
        console.log(prefix + str.replace(/\n/g, "\n" + " ".repeat(prefix.length) + "> "));
    }
}
