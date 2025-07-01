#!/bin/bash

SCREEN_NAME="socat_screen"

if screen -list | grep -q "$SCREEN_NAME"; then
    screen -S "$SCREEN_NAME" -X quit
    sleep 1
    # Nettoie les screens morts
    screen -wipe > /dev/null
fi

screen -dmS "$SCREEN_NAME" socat -d -d pty,rawer,echo=0 pty,rawer,echo=0