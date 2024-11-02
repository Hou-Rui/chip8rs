pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

import chip8.backend

ApplicationWindow {
    id: root
    readonly property int videoWidth: 64
    readonly property int videoHeight: 32
    readonly property int pixelSize: 8
    width: videoWidth * pixelSize
    minimumWidth: width
    maximumWidth: width

    visible: true
    title: "CHIP 8"

    property string loadedFile: ""

    ColumnLayout {
        anchors.fill: parent

        Grid {
            columns: root.videoWidth
            rows: root.videoHeight
            Repeater {
                model: backend.video
                Rectangle {
                    required property bool modelData
                    width: root.pixelSize
                    height: root.pixelSize
                    color: modelData ? "white" : "black"
                }
            }
        }

        GridLayout {
            id: buttonLayout
            rows: 4
            columns: 5
            Layout.fillHeight: true
            Layout.fillWidth: true

            GridButton {
                Layout.column: 0
                Layout.row: 0
                text: "Load File"
                onClicked: loadFileDialog.open()
                FileDialog {
                    id: loadFileDialog
                    onAccepted: {
                        root.loadedFile = root.normalize(file);
                        backend.load(root.loadedFile);
                    }
                }
            }

            GridButton {
                Layout.column: 0
                Layout.row: 1
                text: "Reset"
                onClicked: {
                    const wasRunning = runTimer.running;
                    runTimer.running = false;
                    backend.reset();
                    if (root.loadedFile !== "") {
                        backend.load(root.loadedFile);
                    }
                    runTimer.running = wasRunning;
                }
            }

            GridButton {
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.column: 0
                Layout.row: 2
                text: runTimer.running ? "Stop" : "Start"
                onClicked: {
                    if (!runTimer.running && root.loadedFile === "") {
                        confirmNoFileDialog.visible = true;
                        return;
                    }
                    runTimer.running = !runTimer.running;
                }
                MessageDialog {
                    id: confirmNoFileDialog
                    text: "No ROM file loaded."
                    informativeText: "Are you sure you want to continue?"
                    buttons: MessageDialog.Yes | MessageDialog.No
                    onAccepted: runTimer.running = true
                }
                Timer {
                    id: runTimer
                    interval: 16
                    repeat: true
                    onTriggered: backend.cycle()
                }
            }

            Repeater {
                id: keypadButtons
                model: ["1", "2", "3", "C",
                        "4", "5", "6", "D",
                        "7", "8", "9", "E",
                        "A", "0", "B", "F"]
                GridButton {
                    required property string modelData
                    required property int index
                    readonly property int value: parseInt(modelData, 16)
                    text: modelData
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    Layout.row: 0 + index / 4
                    Layout.column: 1 + index % 4
                    onPressed: backend.key_press(value)
                    onReleased: backend.key_release(value)
                }
            }
        }
    }

    component GridButton: Button {
        Layout.fillHeight: true
        Layout.fillWidth: true
        Layout.preferredHeight: 20
    }

    function normalize(url: string): string {
        return url.replace(/^file:\/\//i, "");
    }

    Backend {
        id: backend
    }
}
