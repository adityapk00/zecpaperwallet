/********************************************************************************
** Form generated from reading UI file 'mainwindow.ui'
**
** Created by: Qt User Interface Compiler version 5.12.3
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAINWINDOW_H
#define UI_MAINWINDOW_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QMenuBar>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QScrollArea>
#include <QtWidgets/QSpacerItem>
#include <QtWidgets/QStatusBar>
#include <QtWidgets/QToolBar>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralWidget;
    QGridLayout *gridLayout;
    QGroupBox *Save;
    QHBoxLayout *horizontalLayout;
    QSpacerItem *horizontalSpacer;
    QPushButton *btnSavePDF;
    QPushButton *btnSaveJSON;
    QGroupBox *Config;
    QGridLayout *gridLayout_3;
    QLabel *label_3;
    QLineEdit *txttaddrs;
    QLineEdit *txtEntropy;
    QLabel *label;
    QLineEdit *txtzaddrs;
    QPushButton *btnGenerate;
    QLabel *label_2;
    QScrollArea *scrollArea;
    QWidget *scroll;
    QVBoxLayout *verticalLayout;
    QMenuBar *menuBar;
    QToolBar *mainToolBar;
    QStatusBar *statusBar;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QString::fromUtf8("MainWindow"));
        MainWindow->resize(927, 930);
        centralWidget = new QWidget(MainWindow);
        centralWidget->setObjectName(QString::fromUtf8("centralWidget"));
        gridLayout = new QGridLayout(centralWidget);
        gridLayout->setSpacing(6);
        gridLayout->setContentsMargins(11, 11, 11, 11);
        gridLayout->setObjectName(QString::fromUtf8("gridLayout"));
        Save = new QGroupBox(centralWidget);
        Save->setObjectName(QString::fromUtf8("Save"));
        horizontalLayout = new QHBoxLayout(Save);
        horizontalLayout->setSpacing(6);
        horizontalLayout->setContentsMargins(11, 11, 11, 11);
        horizontalLayout->setObjectName(QString::fromUtf8("horizontalLayout"));
        horizontalSpacer = new QSpacerItem(40, 20, QSizePolicy::Expanding, QSizePolicy::Minimum);

        horizontalLayout->addItem(horizontalSpacer);

        btnSavePDF = new QPushButton(Save);
        btnSavePDF->setObjectName(QString::fromUtf8("btnSavePDF"));

        horizontalLayout->addWidget(btnSavePDF);

        btnSaveJSON = new QPushButton(Save);
        btnSaveJSON->setObjectName(QString::fromUtf8("btnSaveJSON"));

        horizontalLayout->addWidget(btnSaveJSON);


        gridLayout->addWidget(Save, 2, 0, 1, 1);

        Config = new QGroupBox(centralWidget);
        Config->setObjectName(QString::fromUtf8("Config"));
        gridLayout_3 = new QGridLayout(Config);
        gridLayout_3->setSpacing(6);
        gridLayout_3->setContentsMargins(11, 11, 11, 11);
        gridLayout_3->setObjectName(QString::fromUtf8("gridLayout_3"));
        label_3 = new QLabel(Config);
        label_3->setObjectName(QString::fromUtf8("label_3"));

        gridLayout_3->addWidget(label_3, 1, 0, 1, 1);

        txttaddrs = new QLineEdit(Config);
        txttaddrs->setObjectName(QString::fromUtf8("txttaddrs"));

        gridLayout_3->addWidget(txttaddrs, 0, 3, 1, 1);

        txtEntropy = new QLineEdit(Config);
        txtEntropy->setObjectName(QString::fromUtf8("txtEntropy"));

        gridLayout_3->addWidget(txtEntropy, 1, 1, 1, 3);

        label = new QLabel(Config);
        label->setObjectName(QString::fromUtf8("label"));

        gridLayout_3->addWidget(label, 0, 0, 1, 1);

        txtzaddrs = new QLineEdit(Config);
        txtzaddrs->setObjectName(QString::fromUtf8("txtzaddrs"));

        gridLayout_3->addWidget(txtzaddrs, 0, 1, 1, 1);

        btnGenerate = new QPushButton(Config);
        btnGenerate->setObjectName(QString::fromUtf8("btnGenerate"));

        gridLayout_3->addWidget(btnGenerate, 2, 0, 1, 1);

        label_2 = new QLabel(Config);
        label_2->setObjectName(QString::fromUtf8("label_2"));

        gridLayout_3->addWidget(label_2, 0, 2, 1, 1);


        gridLayout->addWidget(Config, 0, 0, 1, 1);

        scrollArea = new QScrollArea(centralWidget);
        scrollArea->setObjectName(QString::fromUtf8("scrollArea"));
        scrollArea->setStyleSheet(QString::fromUtf8(""));
        scrollArea->setWidgetResizable(true);
        scroll = new QWidget();
        scroll->setObjectName(QString::fromUtf8("scroll"));
        scroll->setGeometry(QRect(0, 0, 907, 637));
        verticalLayout = new QVBoxLayout(scroll);
        verticalLayout->setSpacing(6);
        verticalLayout->setContentsMargins(11, 11, 11, 11);
        verticalLayout->setObjectName(QString::fromUtf8("verticalLayout"));
        scrollArea->setWidget(scroll);

        gridLayout->addWidget(scrollArea, 1, 0, 1, 1);

        MainWindow->setCentralWidget(centralWidget);
        menuBar = new QMenuBar(MainWindow);
        menuBar->setObjectName(QString::fromUtf8("menuBar"));
        menuBar->setGeometry(QRect(0, 0, 927, 22));
        MainWindow->setMenuBar(menuBar);
        mainToolBar = new QToolBar(MainWindow);
        mainToolBar->setObjectName(QString::fromUtf8("mainToolBar"));
        MainWindow->addToolBar(Qt::TopToolBarArea, mainToolBar);
        statusBar = new QStatusBar(MainWindow);
        statusBar->setObjectName(QString::fromUtf8("statusBar"));
        MainWindow->setStatusBar(statusBar);
        QWidget::setTabOrder(txtzaddrs, txttaddrs);
        QWidget::setTabOrder(txttaddrs, txtEntropy);
        QWidget::setTabOrder(txtEntropy, btnGenerate);
        QWidget::setTabOrder(btnGenerate, btnSavePDF);
        QWidget::setTabOrder(btnSavePDF, btnSaveJSON);
        QWidget::setTabOrder(btnSaveJSON, scrollArea);

        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QApplication::translate("MainWindow", "Zec Sapling Paper Wallet", nullptr));
        Save->setTitle(QString());
        btnSavePDF->setText(QApplication::translate("MainWindow", "Save as PDF", nullptr));
        btnSaveJSON->setText(QApplication::translate("MainWindow", "Save as JSON", nullptr));
        Config->setTitle(QApplication::translate("MainWindow", "Config", nullptr));
        label_3->setText(QApplication::translate("MainWindow", "Additional Entropy", nullptr));
        txttaddrs->setText(QApplication::translate("MainWindow", "0", nullptr));
        label->setText(QApplication::translate("MainWindow", "Number of z addresses", nullptr));
        txtzaddrs->setText(QApplication::translate("MainWindow", "1", nullptr));
        btnGenerate->setText(QApplication::translate("MainWindow", "Generate Wallets", nullptr));
        label_2->setText(QApplication::translate("MainWindow", "Number of t addresses", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
