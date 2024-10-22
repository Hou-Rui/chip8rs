import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

import Chip8

ApplicationWindow {
    id: root
    width: 640
    height: 320
	visible: true
    title: "CHIP8"

	ColumnLayout {
		Button {
			text: "Test"
			onClicked: chip8.test()
		}

		Grid {
			columns: 64
			rows: 32
			Repeater {
				model: chip8.output

				Rectangle {
					width: 8
					height: 8
					color: modelData ? "grey" : "white"
				}
			}
		}
	}

	Chip8 {
		id: chip8
	}
}