import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

// Provides basic features needed for all kirigami applications
ApplicationWindow {
    id: root
    width: 640
    height: 320

    title: "CHIP8"

	Grid {
		columns: 64
		rows: 32
		
	}
}