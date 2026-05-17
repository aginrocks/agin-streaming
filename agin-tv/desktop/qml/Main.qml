import QtQuick 6.8
import QtQuick.Window 6.8
import QtQuick.VirtualKeyboard 6.8
import QtQuick.Controls 6.8
import QtQuick.Layouts 6.8
import AginTV

Window {
    id: window
    width: 960
    height: 540
    visible: true
    title: qsTr("Agin TV Navigation Demo")
    color: Theme.colors.background

    property string source: "https://image.tmdb.org/t/p/original/6bzabqH399ioM3nZScwZtzGaHIy.jpg"
    property real weight: 400

    Behavior on weight {
        NumberAnimation {
            duration: 300
        }
    }

    Fonts {}

    Hero {
        activeSource: window.source
    }

    TabBar {
        anchors {
            top: parent.top
            right: parent.right
            topMargin: Theme.safeArea.top
            rightMargin: Theme.safeArea.right
        }
    }

    Item {
        anchors.fill: parent
        anchors.margins: Theme.spacing.s(12)

        ColumnLayout {
            spacing: Theme.spacing.s(4)

            Display {
                text: "Agin TV"
            }

            Text {
                id: myText
                text: "09:41"
                color: "white"
                font.family: "Google Sans Flex"
                // font.weight: window.weight
                font.pointSize: 24
                font.variableAxes: {
                    "ROND": 100,
                    "GRAD": 0,
                    "slnt": 0,
                    "wdth": 100,
                    "wght": window.weight
                }
            }

            Button {
                text: ":-)"
                onClicked: {
                    if (window.weight === 700) {
                        window.weight = 400;
                    } else {
                        window.weight = 700;
                    }
                }
            }

            Button {
                text: "Toggle Source"
                onClicked: {
                    window.source = window.source.includes("rAiYTfKGqDCRIIqo664sY9XZIvQ") ? "https://image.tmdb.org/t/p/original/rAiYTfKGqDCRIIqo664sY9XZIvQ.jpg" : "https://image.tmdb.org/t/p/original/rAiYTfKGqDCRIIqo664sY9XZIvQ.jpg";
                }
            }

            RowLayout {
                spacing: Theme.spacing.s(4)

                Cover {
                    source: "https://image.tmdb.org/t/p/w1280/rAiYTfKGqDCRIIqo664sY9XZIvQ.jpg"
                    Navigable.canNavigate: true
                    isSelected: Navigable.hasFocus
                }

                Cover {
                    source: "https://image.tmdb.org/t/p/w1280/iN41Ccw4DctL8npfmYg1j5Tr1eb.jpg"
                    Navigable.canNavigate: true
                    isSelected: Navigable.hasFocus
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
