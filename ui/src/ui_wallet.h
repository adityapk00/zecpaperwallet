/********************************************************************************
** Form generated from reading UI file 'wallet.ui'
**
** Created by: Qt User Interface Compiler version 5.12.3
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_WALLET_H
#define UI_WALLET_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QFrame>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QSpacerItem>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>
#include "qrcodelabel.h"

QT_BEGIN_NAMESPACE

class Ui_WalletWidget
{
public:
    QGridLayout *gridLayout;
    QFrame *line;
    QVBoxLayout *verticalLayout_3;
    QSpacerItem *verticalSpacer_3;
    QLabel *label_3;
    QLabel *lblPrivateKey;
    QSpacerItem *verticalSpacer_4;
    QVBoxLayout *verticalLayout;
    QSpacerItem *verticalSpacer;
    QLabel *label_2;
    QLabel *lblAddress;
    QSpacerItem *verticalSpacer_2;
    QRCodeLabel *qrPrivateKey;
    QRCodeLabel *qrAddress;

    void setupUi(QWidget *WalletWidget)
    {
        if (WalletWidget->objectName().isEmpty())
            WalletWidget->setObjectName(QString::fromUtf8("WalletWidget"));
        WalletWidget->resize(847, 533);
        gridLayout = new QGridLayout(WalletWidget);
        gridLayout->setObjectName(QString::fromUtf8("gridLayout"));
        line = new QFrame(WalletWidget);
        line->setObjectName(QString::fromUtf8("line"));
        line->setFrameShape(QFrame::HLine);
        line->setFrameShadow(QFrame::Sunken);

        gridLayout->addWidget(line, 1, 0, 1, 4);

        verticalLayout_3 = new QVBoxLayout();
        verticalLayout_3->setObjectName(QString::fromUtf8("verticalLayout_3"));
        verticalSpacer_3 = new QSpacerItem(20, 40, QSizePolicy::Minimum, QSizePolicy::Expanding);

        verticalLayout_3->addItem(verticalSpacer_3);

        label_3 = new QLabel(WalletWidget);
        label_3->setObjectName(QString::fromUtf8("label_3"));
        label_3->setStyleSheet(QString::fromUtf8("font-weight: bold;"));

        verticalLayout_3->addWidget(label_3);

        lblPrivateKey = new QLabel(WalletWidget);
        lblPrivateKey->setObjectName(QString::fromUtf8("lblPrivateKey"));
        lblPrivateKey->setWordWrap(true);

        verticalLayout_3->addWidget(lblPrivateKey);

        verticalSpacer_4 = new QSpacerItem(20, 40, QSizePolicy::Minimum, QSizePolicy::Expanding);

        verticalLayout_3->addItem(verticalSpacer_4);


        gridLayout->addLayout(verticalLayout_3, 2, 0, 1, 3);

        verticalLayout = new QVBoxLayout();
        verticalLayout->setObjectName(QString::fromUtf8("verticalLayout"));
        verticalSpacer = new QSpacerItem(20, 40, QSizePolicy::Minimum, QSizePolicy::Expanding);

        verticalLayout->addItem(verticalSpacer);

        label_2 = new QLabel(WalletWidget);
        label_2->setObjectName(QString::fromUtf8("label_2"));
        label_2->setStyleSheet(QString::fromUtf8("font-weight: bold;"));
        label_2->setAlignment(Qt::AlignRight|Qt::AlignTrailing|Qt::AlignVCenter);

        verticalLayout->addWidget(label_2);

        lblAddress = new QLabel(WalletWidget);
        lblAddress->setObjectName(QString::fromUtf8("lblAddress"));
        lblAddress->setAlignment(Qt::AlignRight|Qt::AlignTrailing|Qt::AlignVCenter);
        lblAddress->setWordWrap(true);

        verticalLayout->addWidget(lblAddress);

        verticalSpacer_2 = new QSpacerItem(20, 40, QSizePolicy::Minimum, QSizePolicy::Expanding);

        verticalLayout->addItem(verticalSpacer_2);


        gridLayout->addLayout(verticalLayout, 0, 1, 1, 3);

        qrPrivateKey = new QRCodeLabel(WalletWidget);
        qrPrivateKey->setObjectName(QString::fromUtf8("qrPrivateKey"));

        gridLayout->addWidget(qrPrivateKey, 2, 3, 1, 1);

        qrAddress = new QRCodeLabel(WalletWidget);
        qrAddress->setObjectName(QString::fromUtf8("qrAddress"));
        qrAddress->setMouseTracking(false);

        gridLayout->addWidget(qrAddress, 0, 0, 1, 1);


        retranslateUi(WalletWidget);

        QMetaObject::connectSlotsByName(WalletWidget);
    } // setupUi

    void retranslateUi(QWidget *WalletWidget)
    {
        WalletWidget->setWindowTitle(QApplication::translate("WalletWidget", "Form", nullptr));
        label_3->setText(QApplication::translate("WalletWidget", "Private Key", nullptr));
        lblPrivateKey->setText(QApplication::translate("WalletWidget", "TextLabel", nullptr));
        label_2->setText(QApplication::translate("WalletWidget", "Address", nullptr));
        lblAddress->setText(QApplication::translate("WalletWidget", "TextLabel", nullptr));
        qrPrivateKey->setText(QApplication::translate("WalletWidget", "TextLabel", nullptr));
        qrAddress->setText(QApplication::translate("WalletWidget", "TextLabel", nullptr));
    } // retranslateUi

};

namespace Ui {
    class WalletWidget: public Ui_WalletWidget {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_WALLET_H
