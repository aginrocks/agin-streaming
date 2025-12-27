import QtQuick 6.8
import QtQuick.Window 6.8
import QtQuick.VirtualKeyboard 6.8
import QtQuick.Controls 6.8
import QtQuick.Layouts 6.8
import "components"

Window {
    id: window
    width: 960
    height: 540
    visible: true
    title: qsTr("Agin TV Navigation Demo")
    color: Theme.colors.background

    Item {
        anchors.fill: parent
        anchors.margins: Theme.spacing.s(12)

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
