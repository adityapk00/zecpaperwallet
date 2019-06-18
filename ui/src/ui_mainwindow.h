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
#include <QtWidgets/QCheckBox>
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
    QGroupBox *groupBox;
    QGridLayout *gridLayout_2;
    QGroupBox *Config;
    QGridLayout *gridLayout_3;
    QLineEdit *txttaddrs;
    QLineEdit *txtzaddrs;
    QLabel *label;
    QLabel *label_3;
    QLabel *label_2;
    QLineEdit *lineEdit;
    QCheckBox *checkBox;
    QScrollArea *scrollArea;
    QWidget *scroll;
    QVBoxLayout *verticalLayout;
    QGroupBox *Save;
    QHBoxLayout *horizontalLayout;
    QSpacerItem *horizontalSpacer;
    QPushButton *pushButton;
    QPushButton *pushButton_2;
    QMenuBar *menuBar;
    QToolBar *mainToolBar;
    QStatusBar *statusBar;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName(QString::fromUtf8("MainWindow"));
        MainWindow->resize(805, 609);
        centralWidget = new QWidget(MainWindow);
        centralWidget->setObjectName(QString::fromUtf8("centralWidget"));
        gridLayout = new QGridLayout(centralWidget);
        gridLayout->setSpacing(6);
        gridLayout->setContentsMargins(11, 11, 11, 11);
        gridLayout->setObjectName(QString::fromUtf8("gridLayout"));
        groupBox = new QGroupBox(centralWidget);
        groupBox->setObjectName(QString::fromUtf8("groupBox"));
        gridLayout_2 = new QGridLayout(groupBox);
        gridLayout_2->setSpacing(6);
        gridLayout_2->setContentsMargins(11, 11, 11, 11);
        gridLayout_2->setObjectName(QString::fromUtf8("gridLayout_2"));
        Config = new QGroupBox(groupBox);
        Config->setObjectName(QString::fromUtf8("Config"));
        gridLayout_3 = new QGridLayout(Config);
        gridLayout_3->setSpacing(6);
        gridLayout_3->setContentsMargins(11, 11, 11, 11);
        gridLayout_3->setObjectName(QString::fromUtf8("gridLayout_3"));
        txttaddrs = new QLineEdit(Config);
        txttaddrs->setObjectName(QString::fromUtf8("txttaddrs"));

        gridLayout_3->addWidget(txttaddrs, 0, 3, 1, 1);

        txtzaddrs = new QLineEdit(Config);
        txtzaddrs->setObjectName(QString::fromUtf8("txtzaddrs"));

        gridLayout_3->addWidget(txtzaddrs, 0, 1, 1, 1);

        label = new QLabel(Config);
        label->setObjectName(QString::fromUtf8("label"));

        gridLayout_3->addWidget(label, 0, 0, 1, 1);

        label_3 = new QLabel(Config);
        label_3->setObjectName(QString::fromUtf8("label_3"));

        gridLayout_3->addWidget(label_3, 1, 0, 1, 1);

        label_2 = new QLabel(Config);
        label_2->setObjectName(QString::fromUtf8("label_2"));

        gridLayout_3->addWidget(label_2, 0, 2, 1, 1);

        lineEdit = new QLineEdit(Config);
        lineEdit->setObjectName(QString::fromUtf8("lineEdit"));

        gridLayout_3->addWidget(lineEdit, 1, 1, 1, 3);

        checkBox = new QCheckBox(Config);
        checkBox->setObjectName(QString::fromUtf8("checkBox"));
        checkBox->setChecked(true);

        gridLayout_3->addWidget(checkBox, 2, 0, 1, 4);


        gridLayout_2->addWidget(Config, 0, 0, 1, 1);

        scrollArea = new QScrollArea(groupBox);
        scrollArea->setObjectName(QString::fromUtf8("scrollArea"));
        scrollArea->setWidgetResizable(true);
        scroll = new QWidget();
        scroll->setObjectName(QString::fromUtf8("scroll"));
        scroll->setGeometry(QRect(0, 0, 761, 274));
        verticalLayout = new QVBoxLayout(scroll);
        verticalLayout->setSpacing(6);
        verticalLayout->setContentsMargins(11, 11, 11, 11);
        verticalLayout->setObjectName(QString::fromUtf8("verticalLayout"));
        scrollArea->setWidget(scroll);

        gridLayout_2->addWidget(scrollArea, 1, 0, 1, 1);


        gridLayout->addWidget(groupBox, 0, 0, 1, 1);

        Save = new QGroupBox(centralWidget);
        Save->setObjectName(QString::fromUtf8("Save"));
        horizontalLayout = new QHBoxLayout(Save);
        horizontalLayout->setSpacing(6);
        horizontalLayout->setContentsMargins(11, 11, 11, 11);
        horizontalLayout->setObjectName(QString::fromUtf8("horizontalLayout"));
        horizontalSpacer = new QSpacerItem(40, 20, QSizePolicy::Expanding, QSizePolicy::Minimum);

        horizontalLayout->addItem(horizontalSpacer);

        pushButton = new QPushButton(Save);
        pushButton->setObjectName(QString::fromUtf8("pushButton"));

        horizontalLayout->addWidget(pushButton);

        pushButton_2 = new QPushButton(Save);
        pushButton_2->setObjectName(QString::fromUtf8("pushButton_2"));

        horizontalLayout->addWidget(pushButton_2);


        gridLayout->addWidget(Save, 1, 0, 1, 1);

        MainWindow->setCentralWidget(centralWidget);
        menuBar = new QMenuBar(MainWindow);
        menuBar->setObjectName(QString::fromUtf8("menuBar"));
        menuBar->setGeometry(QRect(0, 0, 805, 22));
        MainWindow->setMenuBar(menuBar);
        mainToolBar = new QToolBar(MainWindow);
        mainToolBar->setObjectName(QString::fromUtf8("mainToolBar"));
        MainWindow->addToolBar(Qt::TopToolBarArea, mainToolBar);
        statusBar = new QStatusBar(MainWindow);
        statusBar->setObjectName(QString::fromUtf8("statusBar"));
        MainWindow->setStatusBar(statusBar);

        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QApplication::translate("MainWindow", "Zec Sapling Paper Wallet", nullptr));
        groupBox->setTitle(QApplication::translate("MainWindow", "Paper Wallet Generator", nullptr));
        Config->setTitle(QApplication::translate("MainWindow", "Config", nullptr));
        label->setText(QApplication::translate("MainWindow", "Number of z addresses", nullptr));
        label_3->setText(QApplication::translate("MainWindow", "Additional Entropy", nullptr));
        label_2->setText(QApplication::translate("MainWindow", "Number of t addresses", nullptr));
        checkBox->setText(QApplication::translate("MainWindow", "Generate from a single HD Key", nullptr));
        Save->setTitle(QApplication::translate("MainWindow", "Save", nullptr));
        pushButton->setText(QApplication::translate("MainWindow", "Save as PDF", nullptr));
        pushButton_2->setText(QApplication::translate("MainWindow", "Save as JSON", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
