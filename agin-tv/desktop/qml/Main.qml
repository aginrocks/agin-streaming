import QtQuick 6.8
import QtQuick.Window 6.8
import QtQuick.VirtualKeyboard 6.8
import QtQuick.Controls 6.8
import QtQuick.Layouts 6.8
import AginTV
import "js/request.js" as XHR

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
                Layout.fillWidth: true

                TextField {
                    id: urlTextField
                    text: "https://api.themoviedb.org/3/search/movie?query=Jack+Reacher"
                    Layout.fillWidth: true
                }
                TextField {
                    id: urlTokenField
                    text: preferences.tmdbTOKEN
                    Layout.fillWidth: true
                    onTextChanged: {
                        preferences.tmdbTOKEN = urlTokenField.text;
                    }
                }
                Button {
                    text: qsTr("Sent")
                    onClicked: {
                        XHR.sendRequest(urlTextField.text, urlTokenField.text, function (response) {
                            const json = JSON.parse(response.content);

                            var component = Qt.createComponent("components/Cover.qml");
                            for (const i of json.results) {
                                if (!i.poster_path)
                                    continue;
                                var object = component.createObject(container);
                                if (!object) {
                                    console.error("Failed to create Cover.qml object");
                                    return;
                                }
                                object.source = `https://image.tmdb.org/t/p/original${i.poster_path}`;
                            }
                        });
                    }
                }
            }
            GridLayout {
                id: container
                columnSpacing: Theme.spacing.s(4)
                rows: 5
                columns: 5
                //spacing: Theme.spacing.s(4)
                Layout.fillWidth: true
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
