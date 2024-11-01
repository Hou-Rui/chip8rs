import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

import Chip8

ApplicationWindow {
    id: root
    width: 512
    height: 300
	visible: true
    title: "CHIP8"

	property string loadedFile: ""

	ColumnLayout {
		RowLayout {
			Button {
				text: "Load File"
				onClicked: loadFileDialog.open()
				FileDialog {
					id: loadFileDialog
					onAccepted: {
						root.loadedFile = root.normalize(file);
						chip8.load(root.loadedFile);
					}
				}
			}
			Button {
				text: "Reset"
				onClicked: {
					const wasRunning = runTimer.running;
					runTimer.running = false;
					chip8.reset();
					if (root.loadedFile !== "") {
						chip8.load(root.loadedFile);
					}
					runTimer.running = wasRunning;
				}
			}
			Button {
				text: runTimer.running ? "Stop" : "Start"
				onClicked: {
					if (!runTimer.running && root.loadedFile === "") {
						confirmNoFileDialog.visible = true;
						return;
					}
					runTimer.running = !runTimer.running
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
					interval: 20
					repeat: true
					onTriggered: chip8.cycle()
				}
			}
		}

		Grid {
			columns: 64
			rows: 32
			Repeater {
				model: chip8.video
				Rectangle {
					required property bool modelData
					width: 8
					height: 8
					color: modelData ? "grey" : "black"
				}
			}
		}
	}

	function normalize(url: string): string {
		return url.replace(/^file:\/\//i, "");
	}

	Chip8 {
		id: chip8
	}
}
