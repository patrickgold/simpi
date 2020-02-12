#!/bin/bash

# Aquire root privileges
[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"

# Script-scope global variables
app_id="simpi"
app_name="SimPi Broker"
prog_files_dir="/opt/$app_id"
start_menu_dir="/usr/share/applications"

simpi_uninstall () {
    echo -e "--< Simpi Uninstaller >--\n"

    echo "This script will uninstall SimPi broker and the wpisim libraries"
    echo "from this PC. All data in ~/.simpi will be kept and can be manually"
    echo -e "removed if desired.\n"
    read -r -p "Are you sure you want to uninstall? [y/N] " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            # User wants to uninstall :(
            ;;
        *)
            echo "Aborting uninstall process..."
            exit 1
            ;;
    esac

    # #0 - Delete folders
    echo "Remove SimPi program files..."
    sudo rm -rf "$prog_files_dir"
    sudo rm "/usr/local/include/wiringPi.h"
    sudo rm "/usr/local/lib/libwpisim.d"
    sudo rm "/usr/local/lib/libwpisim.so"
    echo "Remove menu entry..."
    sudo rm "$start_menu_dir/$app_id.desktop"

    # #1 - Done
    echo
    read -n 1 -s -r -p "Done! Press any key to continue..."
    echo
}

simpi_uninstall
