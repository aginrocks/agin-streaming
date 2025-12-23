import QtQuick
import QtQuick.VirtualKeyboard
import QtQuick.Controls
import QtQuick.Layouts
import AginTV

Window {
    id: window
    width: 960
    height: 540
    visible: true
    title: qsTr("Agin TV Navigation Demo")
    color: "#0d1117"

    property color focusColor: "#00d1b2"
    property color cardColor: "#161b22"
    property color textColor: "#e6edf3"

    ColumnLayout {
        id: layout
        anchors.fill: parent
        anchors.margins: 32
        spacing: 24

        Label {
            text: "Use keyboard arrows or controller to navigate"
            color: window.textColor
            font.pixelSize: 20
            font.bold: true
        }

        RowLayout {
            id: menuRow
            spacing: 16

            Repeater {
                model: ["Home", "Movies", "Series", "Sports", "Kids"]

                delegate: Button {
                    id: control
                    required property var modelData
                    text: modelData
                    flat: true
                    implicitWidth: 120
                    implicitHeight: 48
                    Navigable.canNavigate: true

                    background: Rectangle {
                        radius: 8
                        color: Navigable.hasFocus ? "red" : "transparent"
                        border.width: Navigable.hasFocus ? 0 : 1
                    }

                    contentItem: Text {
                        text: control.text
                        anchors.centerIn: parent
                        color: Navigable.hasFocus ? "#0d1117" : window.textColor
                        font.pixelSize: 16
                        font.bold: true
                    }
                }
            }
        }

        GridLayout {
            id: contentGrid
            columns: 4
            rowSpacing: 16
            columnSpacing: 16

            Repeater {
                model: 12

                delegate: Rectangle {
                    width: 180
                    height: 120
                    radius: 16
                    color: Navigable.hasFocus ? window.focusColor : window.cardColor
                    border.color: Navigable.hasFocus ? window.focusColor : "#30363d"
                    border.width: Navigable.hasFocus ? 0 : 1
                    Navigable.canNavigate: true

                    Column {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 8

                        Text {
                            text: "Card " + (index + 1)
                            color: window.textColor
                            font.pixelSize: 18
                            font.bold: true
                        }

                        Text {
                            text: "Description for item " + (index + 1)
                            color: "#8b949e"
                            font.pixelSize: 14
                            wrapMode: Text.WordWrap
                        }
                    }
                }
            }
        }

        RowLayout {
            id: actionsRow
            spacing: 16

            Repeater {
                model: ["Play", "Add to List", "Details"]

                delegate: Button {
                    id: actionButton
                    text: modelData
                    implicitWidth: 140
                    implicitHeight: 48
                    Navigable.canNavigate: true

                    background: Rectangle {
                        radius: 10
                        color: Navigable.hasFocus ? window.focusColor : "transparent"
                        border.color: window.focusColor
                        border.width: Navigable.hasFocus ? 0 : 1
                    }

                    contentItem: Text {
                        text: actionButton.text
                        anchors.centerIn: parent
                        color: Navigable.hasFocus ? "#0d1117" : window.textColor
                        font.pixelSize: 16
                        font.bold: true
                    }
                }
            }
        }
    }

    InputPanel {
        id: inputPanel
        z: 99
        y: window.height
        width: window.width

        states: State {
            name: "visible"
            when: inputPanel.active
            PropertyChanges {
                inputPanel.y: window.height - inputPanel.height
            }
        }
        transitions: Transition {
            from: ""
            to: "visible"
            reversible: true
            NumberAnimation {
                properties: "y"
                easing.type: Easing.InOutQuad
            }
        }
    }
}
