import QtQuick 6.8
import QtQuick.Window 6.8
import QtQuick.Controls 6.8
import QtQuick.Layouts 6.8
import AginTV

Item {
    implicitWidth: column.implicitWidth
    implicitHeight: column.implicitHeight
    ColumnLayout {
        id: column
        anchors.fill: parent

        Title {
            text: "Login"
        }

        TextField {
            placeholderText: "Username"
        }

        TextField {
            placeholderText: "Password"
            echoMode: TextInput.Password
        }

        Button {
            Layout.alignment: Qt.AlignHCenter
            text: "Submit"
            onClicked: {
                print(loginController.login);
            }
        }
    }
}
