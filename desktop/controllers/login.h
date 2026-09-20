
#pragma once

#include <QtNetwork/qnetworkaccessmanager.h>
#include <QtNetwork/qnetworkreply.h>
#include <QtNetwork/qnetworkrequest.h>
#include <qlogging.h>
#include <qtmetamacros.h>
#include <qtpreprocessorsupport.h>
#include <qtypes.h>

#include <QObject>
#include <QUrl>

namespace UILogic {

class LoginController: public QObject {
public:
    explicit LoginController(QObject* parent = nullptr) :
        QObject(parent),
        manager(new QNetworkAccessManager(this)) {}

    Q_PROPERTY(quint32 isLogged READ isLogged NOTIFY onLogin)

    Q_OBJECT
    Q_INVOKABLE void login() {
        QNetworkRequest request(QUrl("https://1.1.1.1"));
        QNetworkReply* reply = manager->get(QNetworkRequest(request));

        connect(reply, &QNetworkReply::finished, this, [=, this]() {
            end_login(reply);
        });
    }

    quint32 isLogged() {
        return _isLogged;
    }

Q_SIGNALS:
    void onLogin(
        const qint32& result
    ); // onLogin: (subject)=> console.log("New message received:", subject)
private:
    QNetworkAccessManager* manager;

    void end_login(QNetworkReply* reply) {
        bool succes = false;
        if (reply->error() == QNetworkReply::NoError) {
            QByteArray response = reply->readAll();
            // you need some custom parser :(
            Q_UNUSED(response)
            succes = true;
        }
        qDebug() << "login ended : " << succes << " !\n";
        _isLogged = succes;
        emit onLogin(succes);
    }

    bool _isLogged = false;
};

} // namespace UILogic
